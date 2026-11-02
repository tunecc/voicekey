# Design

## 目标与非目标

- 目标：把菜单栏托盘图标功能以关键文件最小修改集移植到上游最新基线（upstream/main v1.9.0，`25b3f50`），并在该基线上验证构建成功
- 非目标：不修改本仓库（fork）代码；不携带 fork 其他增强；不修 wtlocal 的 stable 编译问题（上游既有问题，PR 描述附注即可）

## PR 分支拓扑（沿用 upstream-pr-sound-feedback 先例）

- 基线：`upstream/main`（25b3f50，v1.9.0）。已核实：ui.rs 托盘区域自 merge-base（c7e876d）起上游未改动；controller.rs 的 3 处 `set_tray_active` 调用点仍在（约 L414/L490/L505）；`tray-active.png` 在上游仅被 ui.rs L14 引用
- change 工作区：托管 worktree `.worktrees/upstream-pr-menubar-icon`（分支 `comet/upstream-pr-menubar-icon`），仅承载流程产物
- PR 分支：`feat/menubar-wave-icon`，位于独立 git worktree `.worktrees/pr-menubar-icon`，经 `.git/info/exclude` 排除（不污染主工作区 status，也不进入 PR）；全部提取、构建与验证都在 PR worktree 内执行

## 关键文件清单（PR 全部内容）

新增（自 fork main `664bb30` 原样移植）：

| 文件 | 说明 |
| ---- | ---- |
| `Scripts/tray-icons.swift` | 托盘图标生成脚本：44px/18pt alpha 模板绘制，preview 选型 + write 输出单张 tray.png（空闲/录音中共用「录音中」形态） |

替换 / 删除（二进制资产，自 fork main 原样移植）：

| 文件 | 说明 |
| ---- | ---- |
| `app/src-tauri/icons/tray.png` | App 风格单线声波模板图标（含 tray-wave-centering 墨迹居中修正，819 字节） |
| `app/src-tauri/icons/tray-active.png` | 删除（上游 250 字节旧双态图标） |

修改（上游版本 + 仅图标 hunks）：

| 文件 | 图标 hunks |
| ---- | ---- |
| `app/src-tauri/src/ui.rs` | ① 删 L14 `const TRAY_ACTIVE` ② `tray_icon(app, active)` → `tray_icon(app)`，body 改 `Image::from_bytes(TRAY).ok()` ③ `build_tray` 内 `tray_icon(app, false)` → `tray_icon(app)` ④ 整体删除 `pub fn set_tray_active`（7 行） |
| `app/src-tauri/src/controller.rs` | 删 3 行 `ui::set_tray_active(&self.app, true/false);`（两处紧跟 `self.last_voice = Instant::now();` 之后，一处紧跟 `r.stop();` 之后） |

## 明确排除（防泄漏清单）

ui.rs 的 `tray_head()` 及 `tray_menu` 头部文案 hunk 与其单测（属「托盘触发文案」功能）；`Cargo.toml`（rodio/objc2-app-kit）；`Cargo.lock`；`tauri.conf.json`（updater pubkey/endpoints、bundle.resources）；`resources/sounds/`；`sound.rs`；`settings.rs`；`main.rs`/`update.rs`/`hotkey.rs`；前端 `app/src/**`；`crates/`；`docs/`；fork 的其他 icons。

## 工具链与验证（引用项目记忆 stable-1-95-wtlocal-stdarch-neon）

- 上游基线 `crates/wtlocal` 在 stable ≥1.95 下 E0658（unstable `vdotq_s32`），**cargo 验证必须用 nightly 工具链**（nightly 1.101 已验证可编译，无需额外 target-feature）
- 上游无 CI fmt 门禁，`cargo fmt --check` 对纯净上游报数百处 diff，**不适用**
- 验证命令（均在 PR worktree 内、以 `app` 为 cwd 或 workspace 根执行）：
  1. 前端：`npm ci && npm run build`（vite）
  2. Rust：`cargo +nightly check --workspace`、`cargo +nightly test --workspace`
- 泄漏审查：`git diff upstream/main..HEAD --stat` 逐文件核对 = 关键文件清单（5 项）；`grep -rn "TRAY_ACTIVE\|set_tray_active\|tray-active"` 在上游源码树零残留

## PR 形态

- 单提交，提交信息沿用 fork 上的功能命名：`feat(macos): 菜单栏图标改为 App 风格单线声波`
- PR 标题/描述草稿写入本 change 目录 `pr-draft.md`（zh-CN，含功能说明、脚本用法、wtlocal 既有问题附注），PR 创建在归档交付确认后执行
