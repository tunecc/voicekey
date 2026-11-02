# Tasks

## 1. 实现（对应 review 1–7）

- [x] 1.1 D1 移除「识别完成」：`sound.rs` 删 `SoundEvent::Done`/`DEFAULT_DONE` 及分支，`settings.rs` 删 `sound_done`（Default/sanitize/测试），`Audio.svelte` CHIME_ROWS 剩 3 行，`api.ts` 删 `soundDone`。验证：`cargo +nightly check -p voicekey` 0 警告
- [x] 1.2 D2 ducked 门控：`controller.rs` 加 `ducked` 字段（handler 生效置位、显式关/取消/新会话复位），`play_after_restore` 仅置位时等恢复窗口。验证：`cargo +nightly check -p voicekey` 通过
- [x] 1.3 D3 平台音效列表：`sound.rs` 加 `selectable_sounds()`，`main.rs list_sounds` 改用，`settings.rs` sanitize 平台化自愈 + cfg 单测。验证：`cargo +nightly test -p voicekey sound` 相关通过
- [x] 1.4 D4 默认关：`settings.rs` Default `sound_enabled: false`，legacy 测试断言与注释同步。验证：`cargo +nightly test -p voicekey settings` 通过
- [x] 1.5 D5 cancel 作废：`cancel()` 开头递增 `chime_gen`，`ducked` 复位；确认 `abort()` 顺序下失败音不受影响。验证：`cargo +nightly check -p voicekey` 通过
- [x] 1.6 D6 延迟压音量代数：`start()` spawn 捕获 `my_gen`，醒来校验一致才发送。验证：`cargo +nightly check -p voicekey` 通过
- [x] 1.7 D7 资源收敛：删 7 个未用 WAV，重写 `LICENSE.md`（3 行表 + 用途修正），确认 `tauri.conf.json` 通配不变。验证：`ls resources/sounds/` 仅 4 文件
- [x] 1.8 D7 追加（用户反馈，两轮试听迭代）：原 `tink.wav`（tick_004）音色干涩、与 macOS 系统 Tink 差距大。100 个候选频谱匹配不可靠后改人耳试听：第一轮选定 glass_001，复测后用户改选 **glass_005** 并要求名称与音色一致——兜底映射由 `Tink→tink.wav` 改为 `Glass→glass.wav`，Windows 下拉变为 Basso/Glass/Pop，非 macOS 默认开始音 = Glass（macOS 保持 Tink）。`LICENSE.md` 同步。约束：Apple 系统音不可再分发，只能 CC0 近似。验证：用户双平台实测确认 + 池内时长回归测试（glass 44ms）通过

## 2. 验证

- [x] 2.1 `cargo +nightly check -p voicekey`（0 警告）+ `cargo +nightly test --workspace` 全绿。验证：退出码 0
- [x] 2.2 `npm run build`（vite）通过。验证：退出码 0
- [x] 2.3 macOS 打包实机试听 + Windows 实机测试（用户确认，2026-11-02）：macOS 设置组 3 行、关 ducking 时处理音即时、Esc 取消无排队音；Windows 快捷键经旧进程排查后正常、下拉仅 Basso/Glass/Pop 且 Glass=glass_005 音色符合预期。验证：用户实测确认
