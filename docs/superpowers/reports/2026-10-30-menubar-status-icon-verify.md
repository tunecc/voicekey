# 验证报告：menubar-status-icon

- 日期：2026-10-30
- 工作流：tweak（Classic）
- 验证模式：light（规模评估建议 full：任务数 4 > 3；本变更仅视觉资产与生成脚本，无 delta spec、无 API/数据/行为变更，保留 light）
- review_mode：off（本变更为图标资产与生成脚本，无生产逻辑改动，跳过自动代码审查）
- 验证范围：最终选定的 F（App 原形）单线波形图标及生成脚本

## 检查结果

| # | 检查项 | 结果 | 证据 |
| --- | --- | --- | --- |
| 1 | tasks.md 全部完成 | PASS | 4/4 已勾选（`docs/openspec/changes/menubar-status-icon/tasks.md`） |
| 2 | 改动文件与任务描述一致 | PASS | `Scripts/tray-icons.swift`（新增）、`app/src-tauri/icons/tray.png`、`tray-active.png`（修改）；其余为 change 产物。预览图位于 `/tmp/voicekey-tray-preview/`，可由脚本重新生成，不作为提交资产。 |
| 3 | 编译通过 | PASS | `cargo check -p voicekey` 通过（2.72s）；Build guard 的 `cargo build` exit 0。Verify guard 以同一 `cargo build` 绑定检查。 |
| 4 | 相关测试通过 | PASS（N/A） | 本次只替换图标二进制并新增生成脚本，没有图标相关的自动化测试；相关验证为构建成功、PNG 检查与 18pt 预览。 |
| 5 | 无明显安全问题 | PASS | 无硬编码密钥、无网络请求、无新增 unsafe；脚本仅使用 AppKit 绘制并写入指定目录。 |
| 6 | 最终集成代码审查 | PASS（按配置跳过） | change 配置 `review_mode: off`；无生产逻辑 diff，跳过原因已记录。 |
| 7 | 核心场景与边界验证 | PASS | 见下方资产检查与预览说明。 |

## 资产检查

| 资产 | 尺寸 | Alpha | 非透明像素 | 墨迹范围 (x,y) | 最大 Alpha | 彩色像素 |
| --- | --- | --- | --- | --- | --- | --- |
| `tray.png`（空闲） | 44×44 | 有 | 419 | (1,12)–(42,33) | 217 | 0 |
| `tray-active.png`（录音） | 44×44 | 有 | 575 | (0,9)–(43,37) | 255 | 0 |

- 两态均为纯黑 + alpha，无彩色像素，符合 macOS template image 的着色方式。
- 录音态墨迹范围更大、非透明像素更多、最大 alpha 更高，空闲/录音差异可见。
- SHA-256：`tray.png` = `5939189afed2894a53439377a33c34f91f0146c903f4ffa4ad9eb2dccda938f9`；`tray-active.png` = `5fef0ae62e894f8153e3db58275ae2910db01dd8a905976001292b6b842599b1`。

## 预览与场景

- 使用 `swift Scripts/tray-icons.swift preview /tmp/voicekey-tray-preview` 生成 18pt 实际大小、浅色/深色菜单栏和 4× 放大预览：`/tmp/voicekey-tray-preview/contact-sheet.png`。
- 选定 F 的波形参数：1.5 个周期、相位 0.6；空闲态振幅 3.6pt / 线宽 2.4pt / alpha 0.85，录音态振幅 5.2pt / 线宽 2.75pt / alpha 1.0。
- 覆盖场景：浅色与深色菜单栏自动反色、空闲与录音两态、18pt 实际尺寸与 4× 细节。
- 未启动应用进程做菜单栏实机截图；构建已确认 `include_bytes!` 能读取新资产，实机效果需重新构建/重启应用后查看。

## 范围与偏差

- 初版 A/B/C（竖条、底板、药丸）预览后，用户要求改为“一条声波线上下浮动”；据此改为 E/F/G 单线波形，用户选定 F。
- 该方向变更已通过 Comet escalation 决策记录为 `continue`（继续 tweak），范围仍仅限两个图标资产、生成脚本与 change 文档。
- 未修改 `app/src-tauri/src/ui.rs`、菜单结构、状态机或 Swift legacy 实现。

## 结论

7 项轻量验证全部通过，无 CRITICAL/IMPORTANT 问题。可进入归档阶段。
