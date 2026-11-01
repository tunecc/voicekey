# Design: sync-upstream-v1-7-1

## 实现说明

沿用上次同步 v1.7.0（提交 `10a32a1`）的做法：在 `main` 上执行 `git merge upstream/main`，保留双方历史与改动，不使用 rebase（避免重写本地 17 个提交）。

- `app/src-tauri/tauri.conf.json`：上游改第 4 行版本号，本地改 `bundle.resources`，不同 hunk 自动合并，无需手工处理。
- `Cargo.lock`：上游 bump workspace 版本后未提交 lock 更新，合并后需让 cargo 刷新 lock 中 voicekey 各成员的 `1.7.0` → `1.7.1`（运行 `cargo check` 级别的命令即可触发，不执行 `cargo update -w`，避免无关依赖升级）。
- Chatterfly 修复随合并带入，不做二次加工。

## 验证方式

按本仓库既有验证惯例：

- `cargo test -p voicekey`（构建依赖本机 cmake，已具备）
- `cargo check -p voicekey-app`

两项均通过即视为合并未破坏现有功能；本 change 不修改任何本地代码，无需新增测试。
