# 提案：菜单栏图标品牌化重设计

## Why

主线 macOS 应用的菜单栏图标目前是通用的麦克风图形（`tray.png` / `tray-active.png`），与 App 图标「深色圆角底板 + 多层彩色声波」的品牌语言完全脱节，放在菜单栏里辨识度低，用户认为不好看。

## What Changes

- 重新设计并替换 Tauri 主线的两个菜单栏模板图标：
  - `app/src-tauri/icons/tray.png`（空闲）
  - `app/src-tauri/icons/tray-active.png`（录音中）
- 新增可复现的生成脚本 `Scripts/tray-icons.swift`，按菜单栏 18pt 高度输出模板 PNG（44×44），保持 `icon_as_template(true)` 的现有加载方式。
- 生成阶段先渲染 3 个单线波形候选（简洁 S / App 原形 / 实心底板 + 镂空波形），覆盖空闲与录音两种状态以及浅色/深色菜单栏效果；用户选定 F（App 原形）后落地最终资产。
- 不修改 Swift legacy 实现（`Sources/VoiceKey`），不改动菜单栏点击行为、菜单结构或状态切换逻辑。

## Capabilities

纯视觉资产调整，不改变行为与接口，使用 `skip_specs: true`，不新增或修改 spec。

### New Capabilities

无。

### Modified Capabilities

无。

## Impact

- 资产：`app/src-tauri/icons/tray.png`、`app/src-tauri/icons/tray-active.png`
- 脚本：新增 `Scripts/tray-icons.swift`
- 文档：本 change 目录内的 OpenSpec 产物
- 无 API、依赖、数据结构或跨模块变更
