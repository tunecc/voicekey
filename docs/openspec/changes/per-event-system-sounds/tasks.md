# Tasks

## 1. 实现

- [x] 1.1 `sound.rs`：`SYSTEM_SOUNDS` 集合与逐事件取值重构（`gated_id`/`play`/`play_id`/`play_duration` 删固定映射与 WAV 分支逻辑）；默认值改为 Tink/Pop/none/Basso；同步测试。验证：`cargo check -p voicekey` 通过
- [x] 1.2 `settings.rs`：sanitize 校验四字段 ∈ 系统音 ∪ none，非法回退各事件默认；相关测试更新。验证：方案/音效测试通过
- [x] 1.3 `Audio.svelte` + `api.ts` + `main.rs`：四行改逐事件选择器（「不发声」+ 系统铃声）+ 试听；`list_sounds` 返回系统铃声；移除 `simple_scheme`。验证：`npm run build` 通过
- [x] 1.4 delta spec：`specs/audio-feedback/spec.md`（MODIFIED 三条需求），`openspec validate` 通过

## 2. 验证

- [x] 2.1 `cargo test --workspace` 通过。验证：退出码 0
- [ ] 2.2 打包版实机验证：四行选择器可用、试听即所得、改选后立即生效并持久化；默认值 Tink/Pop/不发声/Basso。验证：本机实测记录
