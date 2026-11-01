# Tasks

- [x] 1. `Scripts/tray-icons.swift`：候选 f 的 `centerY: 9.0` → `10.14`，重新生成 `app/src-tauri/icons/tray.png`（44×44）
- [x] 2. 验证：新 PNG 墨迹上/下边距差值 ≤0.2pt；逐像素对比确认仅整体位移（vs env² + centerY 10.14 渲染）
- [ ] 3. `cargo build` 通过
- [ ] 4. 重打 DMG：`dist/VoiceKey-1.8.0-arm64.dmg`（arm64 + Apple Development 签名），核验内嵌新图标
