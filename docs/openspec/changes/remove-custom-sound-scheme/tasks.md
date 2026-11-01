# Tasks

## 1. 实现

- [x] 1.1 `sound.rs`：移除 `SCHEME_CUSTOM` 与 `is_known_scheme`；`gated_id`/`play`/`play_id`/`play_duration` 删除 custom 分支恒走简洁映射；同步测试。验证：`cargo check -p voicekey` 通过
- [x] 1.2 `settings.rs`：默认 `sound_scheme` 改 simple；`sanitize` 无条件强制简洁；两个方案回退测试改为恒简洁断言。验证：`cargo test -p voicekey` 方案相关测试通过
- [x] 1.3 `Audio.svelte`：删除「音效方案」行与自定义分支（含 `sounds`/`listSounds`）；「识别处理」desc 改「识别完成时轻啵一声」、「识别完成」desc 改「识别很快，完成不另发音效」。验证：`npm run build` 通过
- [x] 1.4 delta spec：`specs/audio-feedback/spec.md`（MODIFIED 三条需求），`openspec validate` 通过

## 2. 验证

- [x] 2.1 `cargo test --workspace` 通过。验证：退出码 0
- [ ] 2.2 打包版实机验证：设置页无「音效方案」行与逐事件控件，两行新描述无冲突；提示音行为不变（开始叮、结果就绪啵、失败警示）。验证：本机实测记录
