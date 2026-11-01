# 验证报告：volume-restore-device-churn

日期：2026-11-01 · 工作流：hotfix（light verify）· 提交：c56d114（引擎修复）、db1471b（时序缓解）

## 1. RED 证据（修复前回归测试失败）

`cargo test -p voicekey-platform`（修复前）：

- `restore_survives_device_reenumeration` FAILED — `assertion failed: (backend.volume("speaker") - 0.8).abs() < 0.003`
  （恢复阶段设备重枚举后音量停留在压低值 0.24，即用户报告的「音量消失」）
- `hold_phase_reenumeration_keeps_original_baseline_and_restores` FAILED — `assertion failed: (backend.volume("speaker") - 0.24).abs() < 0.003`
  （持有阶段重枚举后 lease 被丢弃重建，把压低值 0.24 当新基线继续下压——基线棘轮）

其余 16 项既有测试通过，失败原因均对应本 bug 而非环境或测试本身错误。

## 2. GREEN（修复后）

| 检查 | 命令 | 结果 |
| --- | --- | --- |
| 平台引擎单测 | `cargo test -p voicekey-platform` | 18/18 通过（含 4 项新增回归测试） |
| 应用单测 | `cargo test -p voicekey --bins` | 33/33 通过 |
| 全工作区 | `cargo test --workspace` | 全部通过（33+9+18+1，4 项既有 ignored） |
| 构建 | `cargo build --workspace` | 成功；5 个 warning 全部位于 `app/src-tauri/src/sound.rs`，来自工作区中另一进行中工作（remove-custom-sound-scheme 提示音重构）的未提交改动，与本 change 无关；platform crate 零警告 |

新增回归测试覆盖：

1. `restore_survives_device_reenumeration`：恢复阶段同 UID 换 AudioObjectID → 完整还原
2. `hold_phase_reenumeration_keeps_original_baseline_and_restores`：持有阶段重枚举 → 保住原始基线并还原
3. `reenumeration_with_user_change_still_yields`：重枚举窗口内用户手调音量 → 让权不覆盖
4. `disappeared_device_abandons_restore_after_window`：设备 UID 消失 → 有限重试后放弃，状态归于静止

## 3. 真机探针（macOS 实测）

用 /tmp 独立探针工程直接驱动 `OutputVolumeDucker`（系统音量 44%）：

- 修复前基线：56 → 28 → 56 完整还原（证明引擎常规路径本就正常，问题仅在设备环境变化时触发）
- 修复后：44 → 22 → 44 完整还原；「还原中途再次录音」场景 44 → 22 → 44 亦正确，无回归
- 失败注入（设备重枚举/消失）由单测的 FakeBackend 确定性覆盖；真实 CoreAudio 重枚举无法安全人工触发，以单测代验

## 4. 根因消除检查

- `tick_restore` 一次快照失败即永久放弃 → 已改为 UID 重解析自愈 + 有限重试（`DEVICE_LOST_TICKS_MAX`），UID 消失才放弃 ✓
- `output_volume_for` 仅按易变 AudioObjectID 寻址 → 直读失败/ID 被重分配时按 UID 回退（`output_volume_by_uid`）✓
- 引擎全部 snapshot 调用点均有自愈覆盖（tick_active / tick_restore 显式 heal，restore_immediately 在 heal 后执行，advance 的 CAS 用刷新后的 applied）✓
- controller 会话结束立即重启电平表 → 延迟 600ms 经 `Msg::RefreshMeter` 重启，Idle 门控防误触发 ✓；其余 refresh_meter 调用点（设置页开关、arm/disarm 失败路径）不与恢复窗口重叠 ✓

## 5. 遗留与边界

- 进程被强杀/崩溃时音量停在压低值：上游 W3 已知限制，进程内无法兜底，不在本 change 范围
- Windows 端 `output_volume_by_uid` 与直读等价（endpoint id 稳定），未在真机验证；逻辑与 mac 路径对称、由同一组单测覆盖 FakeBackend 行为
