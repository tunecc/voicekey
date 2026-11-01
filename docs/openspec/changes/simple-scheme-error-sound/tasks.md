# Tasks

## 1. 实现

- [x] 1.1 `sound.rs`：`SIMPLE_SCHEME` 追加 `(Error, "error")`；`simple_scheme_system_name` Error 分支返回 `Basso`；同步文档注释。验证：`cargo check -p voicekey` 通过 <!-- comet-task:1ae79b84-ec5b-4ab6-9605-929db1a59262 -->
- [x] 1.2 `Audio.svelte`：简洁方案区块改为按 EVENTS 固定序 + schemeRows 查找渲染（发声行带音效名+试听，未映射显示「不发声」），移除不再使用的 `evInfo`。验证：`npm run build` 通过 <!-- comet-task:6461437e-443a-4c70-9f60-f8e1501dabe8 -->
- [x] 1.3 delta spec：`specs/audio-feedback/spec.md`（MODIFIED 会话事件提示音、提示音设置、内置音效与跨平台一致），`openspec validate` 通过 <!-- comet-task:ff2ed4bb-387e-41a4-8b26-815540e2d158 -->

## 2. 验证

- [x] 2.1 `cargo test --workspace` 通过。验证：退出码 0 <!-- comet-task:46d59362-85d2-49b6-b384-877228135e19 -->
- [x] 2.2 打包版实机验证：失败行显示「警示」且试听播 Basso；简洁方案识别失败时播失败音。验证：本机实测记录 <!-- comet-task:828c9dab-a8c5-48b7-b007-cec05e991115 -->
