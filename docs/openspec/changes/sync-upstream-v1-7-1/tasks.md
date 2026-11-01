# Tasks: sync-upstream-v1-7-1

- [x] 1. 执行 `git merge upstream/main`（merge commit 保留双方历史），确认合并后 `tauri.conf.json` 同时含版本 1.7.1 与本地 `resources/sounds/*` 资源声明，`chatterfly.rs` 为上游修复版
- [x] 2. 刷新 `Cargo.lock` 中 voicekey workspace 成员版本至 1.7.1（仅成员版本字段，不升级第三方依赖）
- [ ] 3. 运行 `cargo test -p voicekey` 与 `cargo check -p voicekey-app`，确认全部通过
