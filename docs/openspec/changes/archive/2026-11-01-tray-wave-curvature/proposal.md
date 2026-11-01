# Proposal

## Why

菜单栏托盘图标已改为 App 风格单线声波，但用户反馈线条上下起伏的「曲率」仍与 App 图标不一致。经对 `Scripts/icon.swift`（App 图标生成源码）与 `app/src-tauri/icons/tray.png` alpha 轮廓的数学拟合确认：托盘波形的包络指数为 1（`sin(πt)¹`），而 App 图标主波形为 2（`sin(πt)²`），导致托盘线条两端偏胖、起伏均匀，App 图标则两端收尖、中段波峰饱满。

同时用户确认：现有「空闲/录音中」双态图标中，「录音中」形态（更大振幅、更粗线宽）更接近完整 App 图标；且希望空闲与录音中**共用同一张图标**，不再需要双态切换。

## What Changes

- `Scripts/tray-icons.swift`：
  - 取自 App 原形的候选（f，及同源变体 g）的 `appWavePath` 包络参数 `envelopePower` 由 `1.0` 改为 `2.0`，与 `Scripts/icon.swift` 主波形的 `pow(sin(πp), 2)` 对齐；
  - `write` 命令改为只输出**单张** `tray.png`，形态采用用户确认的「录音中」参数（振幅 5.2 / 线宽 2.75 / 不透明度 1.0）。
- `app/src-tauri/src/ui.rs`：删除 `TRAY_ACTIVE` 常量与 `set_tray_active`；`tray_icon` 去掉 `active` 参数，恒用单张 `tray.png`。
- `app/src-tauri/src/controller.rs`：删除 3 处 `ui::set_tray_active` 调用（两处开麦置 true、一处停录置 false）。
- 删除 `app/src-tauri/icons/tray-active.png`。

## Capabilities

### New Capabilities

（无——本变更仅调整视觉资产与托盘图标的内部切换逻辑，不引入新能力。）

### Modified Capabilities

（无——现有 4 个 spec（app-update-channel / audio-feedback / hotkey-modes / recording-volume-ducking）均不覆盖菜单栏图标的视觉形状与状态切换，spec 级行为不变。本 change 的 `.openspec.yaml` 设 `skip_specs: true`。）

## Impact

- 受影响文件：`Scripts/tray-icons.swift`、`app/src-tauri/icons/tray.png`、`app/src-tauri/icons/tray-active.png`（删除）、`app/src-tauri/src/ui.rs`、`app/src-tauri/src/controller.rs`
- 行为变化：菜单栏图标不再随录音状态切换外观（录音状态仍由悬浮 HUD 等既有途径呈现）——这是用户明确要求
- 不影响：托盘菜单、事件处理、非 macOS 的窗口图标回退、打包配置（`tray.png` 路径与 44×44 尺寸不变）
