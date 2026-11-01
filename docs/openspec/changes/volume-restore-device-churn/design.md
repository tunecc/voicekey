# 设计：ducking 恢复按 UID 重解析设备并容忍瞬时失败

单一方案，围绕「恢复必须比设备环境变化更顽固」展开；所有权让渡规则全部保留。

## D1 平台层：按 UID 重解析（mac.rs）

新增 `output_volume_by_uid(uid: &str) -> Option<OutputVolumeSnapshot>`：

- 枚举 `kAudioHardwarePropertyDevices` 拿到全部设备对象，逐个读 UID 与目标比对；命中后复用 `output_snapshot` 读出完整快照（携带**当前** native_id）。
- `output_volume_for` 改为：先按租约记录的 native_id 直读（快路径，行为不变）；直读得到 None 或 `same_device` 不匹配（ID 被重分配）时，回退到按 UID 重解析；重解析命中则返回携带新 ID 的快照。
- UID 也找不到（设备拔出/蓝牙断开）才返回 None——此时放弃恢复是正确语义。

Windows（win.rs）`native_id` 恒为 0、endpoint id 稳定，`output_volume_for` 逻辑保持，不引入新路径。

## D2 引擎层：恢复韧性（volume.rs）

`Backend` trait 增加 `resolve_by_uid(&OutputVolumeSnapshot) -> Option<OutputVolumeSnapshot>`（系统实现走 D1；FakeBackend 按 `device_id` 查表返回当前 native_id 的快照）。

`DuckState` 新增 `device_lost_ticks: u32` 与常量 `DEVICE_LOST_TICKS_MAX`：

- `tick_restore`：`snapshot()` 失败不再立即放弃——先 `resolve_by_uid` 重解析；命中则把 `lease.original`/`lease.applied` 的 `native_id` 刷新为新 ID（**值不变**），重置计数并继续本 tick；未命中递增计数，连续超过阈值（约 800ms，覆盖恢复窗口 + 余量）才放弃。任何一次重解析成功清零计数。
- `tick_active` 持有阶段同样接入重解析（原本失败只是重试，现在能在 ID 变化后自愈）；现有「默认设备切走 → restore_immediately 旧设备」分支先经 UID 重解析确认 UID 真的变了才按设备切换处理，避免把「同设备换 ID」误判为切换。
- 所有权判断在刷新 ID 后照旧执行：重解析出的实际值与 `applied` 不匹配（用户在窗口内手动调过音量）仍立即让权，不覆盖用户选择。
- `matches()`/写入 CAS 逻辑不动。

## D3 编排层：电平表重启移出恢复窗口（controller.rs）

`Ctl::run` 中会话结束回到 Idle 触发的 `refresh_meter`，延迟约 600ms（恢复窗口 450ms + 余量）再执行；电平表开关等其他触发点不受影响。`stop_recording` 的关麦顺序保持现状（音频必须立即停采，恢复意图已在关麦前发出），丢音量的兜底由 D1/D2 承担。

## 测试策略

- 单测（FakeBackend）：恢复开始前把设备 `native_id` 换成新值（同 `device_id`）→ 旧实现音量停留压低值（RED），修复后完整还原；覆盖「持有阶段换 ID」「恢复中换 ID」「UID 消失仍放弃」「换 ID 后用户已改音量仍让权」。
- 真机探针（/tmp 独立工程驱动 `OutputVolumeDucker`）复跑 duck → hold → restore 曲线，确认无回归。
- `cargo test -p voicekey-platform` + `cargo test -p voicekey-app` + `cargo build`。

## 风险与回退

- 风险：重解析把「设备已换」误判为「同设备换 ID」。缓解：UID 全局唯一，且所有权值比对在刷新后立即执行，用户改动的音量不会被覆盖。
- 回退：三处改动彼此独立，revert 单个 commit 即可回到现状。
