# Tasks

- [x] 1. 移除 `arm()` 开麦前的 `lower_volume()` 与失败分支的 `restore_volume()`（含注释）
- [x] 2. 移除 `start_recorder()` 预开麦转正分支的 `restore_volume()` 与 `disarm()` 的 `restore_volume()`（含注释）
- [x] 3. 在 `Msg::LowerVolume` 处理器与 `lower_volume()` 注释中固化包络契约：开始音正常音量播完 → 单次压低 → 结束还原；开始音之前不得压低
- [x] 4. 运行 `cargo test -p voicekey`（构建需本机 cmake）与 `cargo check -p voicekey-app`，确认现有测试全部通过
- [x] 5. 手工验证：设置录音音量目标 15 + 外放媒体，长按热键开始录音，确认音量仅在开始音播完后变化 1 次，结束时还原
