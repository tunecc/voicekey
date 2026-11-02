# Tasks

## 上游 PR：菜单栏图标

- [x] 1. 建立独立 PR worktree `.worktrees/pr-menubar-icon`：基于 `upstream/main`（25b3f50）创建分支 `feat/menubar-wave-icon`，并加入 `.git/info/exclude`
- [x] 2. 应用关键文件：新增 `Scripts/tray-icons.swift`（自 fork main 移植）；替换 `app/src-tauri/icons/tray.png`；删除 `app/src-tauri/icons/tray-active.png`；`ui.rs` 四处图标编辑（删 TRAY_ACTIVE、tray_icon 单态化、build_tray 调用点、删 set_tray_active）；`controller.rs` 删 3 处 `ui::set_tray_active` 调用
- [x] 3. 泄漏审查：`git diff upstream/main..HEAD --stat` 逐文件核对等于 5 项关键文件清单；`grep -rn "TRAY_ACTIVE\|set_tray_active\|tray-active"` 零残留；防泄漏清单（tray_head、Cargo.toml、tauri.conf.json、sound.rs、crates/ 等）零混入
- [x] 4. 上游基线构建验证（PR worktree 内）：前端 `npm ci && npm run build`；Rust `cargo +nightly check --workspace` 与 `cargo +nightly test --workspace`
- [x] 5. PR 分支收尾：单提交（`feat(macos): 菜单栏图标改为 App 风格单线声波`），撰写 `pr-draft.md`（标题 + 描述，含 wtlocal 既有问题附注）
