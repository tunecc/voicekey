# 验证报告：volume-restore-device-churn

- 日期：2026-11-01
- 工作流：classic / hotfix 预设（verify_mode: light）
- 提交：c56d114（引擎修复）、db1471b（时序缓解）、后续产物提交
- 变更基线：60152cbd…（main）

## 0. 规模评估与模式说明

`comet state scale`：recommendation=full（任务数 6>3、文件数 9>8）、selected=light。保留 light 的依据：9 个变更文件中 5 个为本 change 的 OpenSpec/状态产物，实际交付源码 4 个（volume.rs、mac.rs、win.rs、controller.rs）；并发模型未变（仍是单 worker 线程 tick 循环，新增逻辑全部在该线程内）；无公共 API、schema、跨模块变更。风险点（并发/设备状态机）已由确定性单测覆盖，详见下文。

## 1. 轻量验证 7 项检查

| # | 检查项 | 结果 | 证据 |
| --- | --- | --- | --- |
| 1 | tasks.md 全部完成 | PASS | T1–T6 全部 `[x]`（tasks.md） |
| 2 | 改动文件与 tasks 一致 | PASS | `git diff --stat 60152cbd...HEAD`：mac.rs/volume.rs/win.rs（T2/T3，win.rs 为设计 D2 要求的 `output_volume_by_uid` 平台对称实现）、controller.rs（T4），其余 5 个为 change 产物；无超出范围的实现文件 |
| 3 | 编译通过 | PASS | `cargo build --workspace` 成功（Exit 0）；5 个 warning 全部位于 `app/src-tauri/src/sound.rs`，来自工作区中另一进行中工作（remove-custom-sound-scheme）的未提交改动，与本 change 无关 |
| 4 | 相关测试通过 | PASS | `cargo test --workspace`：33（app）+9（core）+18（platform，含 4 项新增回归）+1 全部通过，0 失败 |
| 5 | 无明显安全问题 | PASS | 无硬编码密钥；新增 unsafe 仅 mac.rs `output_volume_by_uid` 的 CoreAudio FFI 调用，与既有 FFI 模式一致（属性地址 + GetPropertyData/Size） |
| 6 | 最终集成代码审查 | PASS（跳过自动审查） | `review_mode: off`（hotfix 预设默认），跳过原因已记录；实现者已完成逐 hunk 自查（所有权让渡路径、borrow 结构、CAS 语义保持） |
| 7 | 核心场景与关键边界 | PASS | 见 §2/§3 |

## 2. RED → GREEN 证据

修复前（RED，失败原因对应本 bug）：

- `restore_survives_device_reenumeration` FAILED — `assertion failed: (backend.volume("speaker") - 0.8).abs() < 0.003`（恢复阶段设备重枚举后音量停留在压低值，即用户报告的「音量消失」）
- `hold_phase_reenumeration_keeps_original_baseline_and_restores` FAILED — `assertion failed: (backend.volume("speaker") - 0.24).abs() < 0.003`（持有阶段重枚举后 lease 丢弃重建，压低值被当新基线棘轮下压）

修复后（GREEN）：上述 2 项转绿，另新增 2 项边界测试通过：

- `reenumeration_with_user_change_still_yields`：重枚举窗口内用户手调音量 → 让权不覆盖
- `disappeared_device_abandons_restore_after_window`：设备 UID 消失 → 有限重试后放弃，状态归于静止

## 3. 真机探针（macOS 实测）

/tmp 独立探针工程直接驱动 `OutputVolumeDucker`（系统音量 44%）：

- 修复前基线：56 → 28 → 56 完整还原（证明常规路径本就正常，缺陷仅在设备环境变化时触发）
- 修复后：44 → 22 → 44 完整还原；「还原中途再次录音」44 → 22 → 44 亦正确，无回归
- 设备重枚举/消失的失败注入由 FakeBackend 单测确定性覆盖；真实 CoreAudio 重枚举无法安全人工触发，以单测代验

## 4. 根因消除检查

- `tick_restore` 一次快照失败即永久放弃 → UID 重解析自愈 + 有限重试（`DEVICE_LOST_TICKS_MAX`），UID 消失才放弃 ✓
- `output_volume_for` 仅按易变 AudioObjectID 寻址 → 直读失败/ID 被重分配时按 UID 回退（`output_volume_by_uid`）✓
- 引擎全部 snapshot 调用点均有自愈覆盖（tick_active / tick_restore 显式 heal；restore_immediately 在 heal 后执行；advance 的 CAS 使用刷新后的 applied）✓
- controller 会话结束立即重启电平表 → 延迟 600ms 经 `Msg::RefreshMeter` 重启，Idle 门控防误触发 ✓；其余 refresh_meter 调用点（设置页开关、arm/disarm 失败路径）不与恢复窗口重叠 ✓

## 5. 工作区外部改动说明

验证期间工作区存在另一进行中工作（remove-custom-sound-scheme：Audio.svelte、controller.rs 提示音重构、其 change 目录）的未提交改动，不属于本 change、未纳入本 change 提交；本 change 的测试与构建在该组合工作区上执行并通过（其 WIP 保持可编译）。本 change 的 4 个源码文件已独立成提交，revert 单个提交即可回退。

## 6. 遗留与边界

- 进程被强杀/崩溃时音量停在压低值：上游 W3 已知限制，进程内无法兜底，不在本 change 范围
- Windows 端 `output_volume_by_uid` 未在真机验证；endpoint id 稳定，逻辑与 mac 路径对称，由同一组单测覆盖 FakeBackend 行为
- 后续可选：将本修复（UID 自愈 + 有限重试）整理后反哺上游 J3n5en/voicekey（上游欢迎对 PR #8 的改进另开 PR）
