# Tasks

## 1. 复现与回归测试

- [x] 1.1 在 `crates/core/src/audio/devices.rs` 增加测试辅助（默认输入设备 ID、设备运行状态），并在 `crates/core/src/audio.rs` 增加两条 `#[ignore]` 实机测试；运行 `cargo test -p voicekey-core --lib -- --ignored live_` 确认第 1 条因「停止后设备仍在运行」失败

## 2. 核心修复

- [x] 2.1 `Recorder` 录音线程在 `drop(stream)` 前调用 `stream.pause()`，并注明 cpal 引用环原因；复跑 1.1 的实机测试，确认设备释放
- [x] 2.2 `selected_device()` 增加默认设备名称匹配快路径，改用 `default_input_device()`；运行 `cargo test -p voicekey-core --lib` 确认现有单测与实机测试全部通过

## 3. 电平表生命周期

- [x] 3.1 `Audio.svelte` 与 `Onboard.svelte` 按 `focus` / `blur` 同步电平表开关，失焦停止采集、聚焦恢复；运行 app 前端构建确认通过

## 4. 集成验证

- [x] 4.1 运行 `cargo test -p voicekey-core --lib`（含 `--ignored` 实机测试）与 app 前端构建，确认全部通过；手动核对系统麦克风指示在语音结束、设置页失焦后消失

## Workflow follow-up

- 归档前完成 Comet verify 并记录 `verify_result: pass`，再进入归档确认。
