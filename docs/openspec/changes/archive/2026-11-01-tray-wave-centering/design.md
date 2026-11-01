# Design

## 修复方案

`Scripts/tray-icons.swift` 候选 f（当前在用的单一图标）绘制参数中，`centerY: 9.0` → `centerY: 10.14`：

- 10.14 = 9.0 + 1.14（墨迹中心偏低量的补偿值）；波形整体上移后墨迹范围变为 3.73..14.27（y-up），上下边距各 ≈3.73pt，居中。
- 波谷下探 5.03pt、波峰上探 2.76pt 的固有不对称保留（这是 App 图标的构图特征，不做波形容器改造）。
- 只改候选 f；候选 g（未在用）与其余候选保持不变，保持预览页其他候选的原样。

## RED 证据（修复前复现）

- 公式扫描：波谷 −5.03pt / 波峰 +2.76pt，墨迹 2.59..13.13（y-up），中心偏离 −1.14pt。
- PNG 实测（alpha>0.05，44px/18pt）：上边距 ≈4.5pt / 下边距 ≈2.45pt，与公式一致。

## 验证方法

- 对新 `tray.png` 重新测量墨迹边距（同一脚本、同一阈值），要求上/下边距差值 ≤0.2pt。
- 逐像素对比新 PNG 与内存渲染的「env² + centerY 10.14」波形（复用上一 change 的验证方法），确认仅整体位移、无形状变化。
- `cargo build` 确认应用可构建（include_bytes 资产替换）。
- 重打 DMG：`npx tauri build --target aarch64-apple-darwin`（Apple Development 证书签名）+ `Scripts/dmg.sh`，产物 `dist/VoiceKey-1.8.0-arm64.dmg`。

## 不做的事

- 不改 App 图标（`Scripts/icon.swift`）——其偏沉构图属于大尺寸图标的设计观感，用户未要求改。
- 不改波形频率/相位/包络/振幅/线宽。
- 不加 delta spec。
