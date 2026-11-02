# Proposal：菜单栏图标功能上游 PR 整理

## Why

fork 的菜单栏托盘图标功能已完成并多轮打磨（App 风格单线声波、恒定单一模板图标、墨迹居中），希望以干净的 PR 回馈上游 J3n5en/voicekey。fork main 上还混有大量其他增强（音效反馈、麦克风释放、音量 duck、tray_head 文案等），必须精确提取，只包含图标功能的关键文件，并在上游最新基线上验证构建成功。

## What Changes

在基于 upstream/main（v1.9.0，`25b3f50`）的独立 PR 分支上，仅包含以下关键文件修改：

- **新增** `Scripts/tray-icons.swift`：托盘模板图标生成脚本（macOS，44px/18pt，alpha 模板绘制，write 模式输出单张 tray.png）
- **替换** `app/src-tauri/icons/tray.png`：App 风格单线声波模板图标（fork main 最终形态，含居中修正）
- **删除** `app/src-tauri/icons/tray-active.png`：空闲/录音中双态图标不再需要
- **修改** `app/src-tauri/src/ui.rs`：删除 `TRAY_ACTIVE` 常量；`tray_icon()` 去掉 `active` 参数恒用单一图标；`build_tray` 调用点同步；删除 `set_tray_active()`
- **修改** `app/src-tauri/src/controller.rs`：删除 3 处 `ui::set_tray_active(...)` 调用（录音开始 ×2、录音结束 ×1）

**明确排除的 fork 增强**（不得混入 PR）：ui.rs 的 `tray_head` 文案功能及其测试、Cargo.toml 的 rodio/objc2-app-kit 依赖与 Cargo.lock 变化、tauri.conf.json 的更新源与 resources 改动、音效反馈/麦克风释放/音量 duck 等全部其他功能代码、crates/ 与 docs/ 全部改动。

## Capabilities

### New Capabilities

（无 —— 本 change 不改变本仓库行为，交付物是上游 PR 分支）

### Modified Capabilities

（无 —— fork 自身代码零改动，`.openspec.yaml` 设 `skip_specs: true`）

## Impact

- **上游侧**：`app/src-tauri`（macOS 托盘图标视觉与状态行为）、`Scripts/`、`app/src-tauri/icons/`；托盘从空闲/录音中双态切换变为恒定单一模板图标
- **fork 本地**：不改 fork main 的任何源码；change 工作区（托管 worktree）仅承载流程产物；PR 分支在独立 worktree 中构建与验证
- **交付物**：基于 upstream/main 的 PR 分支（单提交）+ PR 标题/描述草稿；PR 创建本身在归档阶段的交付确认中执行
