# Tasks

- [x] 1. `Scripts/tray-icons.swift`：候选 f 与 g 的 `appWavePath` 包络参数 `envelopePower: 1.0` → `2.0`；`write` 命令改为只输出单张 `tray.png`（振幅 5.2 / 线宽 2.75 / alpha 1.0）
- [x] 2. 重新生成 `tray.png` 并删除 `app/src-tauri/icons/tray-active.png`，确认 `tray.png` 仍为 44×44
- [x] 3. `app/src-tauri/src/ui.rs`：删除 `TRAY_ACTIVE`、`set_tray_active`，`tray_icon` 去掉 `active` 参数；`app/src-tauri/src/controller.rs`：删除 3 处 `ui::set_tray_active` 调用
- [x] 4. `cargo build` 与 `cargo test --workspace` 通过
- [x] 5. 验证：新 `tray.png` alpha 轮廓拟合为 cycles=1.5、phase≈0.6、env^2，对 icon 主波形 (1.5/0.6/env2) 的 RMS 低于改前基线（idle 0.1290 / active 0.1389）；渲染 8× 对比图肉眼复核

> 任务 5 实际执行：质心拟合法对 2.75pt 粗线的判别力不足（env1/env2 RMS 仅差 0.02，粗线平滑偏向低指数），改用更强的逐像素 alpha 对比——`tray.png` vs 内存渲染 env²(1.5/0.6/amp5.2/lw2.75) RMS=0.0087（实质相同），vs env¹ RMS=0.1733，判定资产确为 env² 形态；8× 对比图（/tmp/vk-tray-final.png）肉眼复核一致。
