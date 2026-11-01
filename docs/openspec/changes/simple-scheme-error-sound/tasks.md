# Tasks

## 1. 实现

- [ ] 1.1 `sound.rs`：`SIMPLE_SCHEME` 追加 `(Error, "error")`；`simple_scheme_system_name` Error 分支返回 `Basso`；同步文档注释。验证：`cargo check -p voicekey` 通过
- [ ] 1.2 `Audio.svelte`：简洁方案区块改为按 EVENTS 固定序 + schemeRows 查找渲染（发声行带音效名+试听，未映射显示「不发声」），移除不再使用的 `evInfo`。验证：`npm run build` 通过
- [ ] 1.3 delta spec：`specs/audio-feedback/spec.md`（MODIFIED 会话事件提示音、提示音设置、内置音效与跨平台一致），`openspec validate` 通过

## 2. 验证

- [ ] 2.1 `cargo test --workspace` 通过。验证：退出码 0
- [ ] 2.2 打包版实机验证：失败行显示「警示」且试听播 Basso；简洁方案识别失败时播失败音。验证：本机实测记录
