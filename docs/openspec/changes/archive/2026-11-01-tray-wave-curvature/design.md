# Design

## 根因（已用源码与数学拟合证实）

- App 图标主波形（`Scripts/icon.swift:31-42`，前景层）：`y = mid + sin(p·π·2·1.5 + 0.6) · 230 · pow(sin(πp), 2)`，即 **cycles 1.5、phase 0.6、包络指数 2**。
- 托盘候选 f（`Scripts/tray-icons.swift:129-138`，当前在用，RMS 0.0996 拟合确认）：`appWavePath(cycles: 1.5, phase: 0.6, envelopePower: 1.0)`，即 **cycles 1.5、phase 0.6、包络指数 1**。
- 频率与相位已一致；曲率差异完全来自包络指数：指数 1 的两端振幅衰减慢（左小鼓包峰值 0.32·amp），指数 2 收尖快（左小鼓包 0.10·amp）、中段更饱满——即用户感知的「起伏曲率不一样」。

## 方案

### 1. 波形曲率对齐（env²）

- `Scripts/tray-icons.swift` 候选 f 与 g（同为 App 原形波形派生）的 `envelopePower` 改为 `2.0`；候选 e（1 周期 S 形，非 App 波形派生）保持不变。

### 2. 单图标（去双态）

用户确认「录音中」形态更接近完整图标，且希望两态共用一张图标：

- 形态参数固定为原「录音中」档：振幅 5.2 / 线宽 2.75 / 不透明度 1.0（env² 下峰值系数 0.982→0.968，<1.5%，无需补偿）。
- `write` 命令只写 `tray.png`；`TrayState` 保留供预览对比页使用。
- `ui.rs`：删 `TRAY_ACTIVE`（第 14 行）、`tray_icon(app:active:)` 的 `active` 分支（第 301-307 行）、`set_tray_active`（第 329-336 行）。
- `controller.rs`：删第 406、493 行的 `set_tray_active(.., true)` 与第 509 行的 `set_tray_active(.., false)`。
- 删除 `app/src-tauri/icons/tray-active.png`。

录音状态呈现不受影响：悬浮 HUD 与托盘菜单的暂停态文案均保留，不在本 change 范围。

## 验证思路

- 客观：对新 `tray.png` 的 alpha 轮廓重新做参数拟合，最佳拟合应为 cycles=1.5、phase≈0.6、env^2，且对「icon 主波形 (1.5/0.6/env2)」组合的 RMS 显著低于改前基线（idle 0.1290 / active 0.1389）。
- 主观：渲染 8× 对比图（新托盘 vs 图标主波形 vs 完整图标），肉眼核对起伏形态。
- 构建：`cargo build` 与 `cargo test --workspace`（ui.rs / controller.rs 有改动）。

## 不做的事

- 不改候选 e 的 1 周期设计；不改图标本体（`Scripts/icon.swift`）；不加第二层波形（单线模板图标是既有设计决策）；不动托盘菜单、HUD 与录音状态的其他呈现途径；不动非 macOS 的窗口图标回退。
