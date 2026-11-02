# Tasks

## 1. 实现

- [x] 1.1 `sound.rs`：移除 `SCHEME_CUSTOM` 与 `is_known_scheme`；`gated_id`/`play`/`play_id`/`play_duration` 删除 custom 分支恒走简洁映射；同步测试。验证：`cargo check -p voicekey` 通过
- [x] 1.2 `settings.rs`：默认 `sound_scheme` 改 simple；`sanitize` 无条件强制简洁；两个方案回退测试改为恒简洁断言。验证：`cargo test -p voicekey` 方案相关测试通过
- [x] 1.3 `Audio.svelte`：删除「音效方案」行与自定义分支（含 `sounds`/`listSounds`）；「识别处理」desc 改「识别完成时轻啵一声」、「识别完成」desc 改「识别很快，完成不另发音效」。验证：`npm run build` 通过
- [x] 1.4 delta spec：`specs/audio-feedback/spec.md`（MODIFIED 三条需求），`openspec validate` 通过

## 2. 验证

- [x] 2.1 `cargo test --workspace` 通过。验证：退出码 0
- [x] 2.2 （已取代，无需执行）上游合并 PR 17 后，main 的提示音实现直接采用上游终版：无「音效方案」行、逐事件控件为上游 3 行版（Glass/Pop/Basso 平台过滤）；本 change 的目标已被更完整地实现，实测由 pr17-sound-review-fixes 的双平台验证覆盖
