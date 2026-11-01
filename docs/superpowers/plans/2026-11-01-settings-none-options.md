---
change: settings-none-options
design-doc: docs/openspec/changes/settings-none-options/design.md
base-ref: ff173c0d255caa3e33483a850ad6fb590c40d212
archived-with: 2026-11-01-settings-none-options
---

<!-- comet-task-authority: docs/openspec/changes/settings-none-options/tasks.md -->

# 实施计划：settings-none-options

策略：autonomous + TDD（每个行为任务先写失败测试再实现）+ standard 审查。配置：isolation=current。设计依据：design.md D1–D4；验收：delta specs（audio-feedback / hotkey-modes）。

## 1. 后端：`none` 取值贯通

<!-- comet-task-ref:45612505-c891-42c4-8276-754a92b2342f -->
**1.1 sanitize 保留 `none`**（依赖：无）
- RED：`settings.rs` 测试新增——音效字段设为 `none` 经 sanitize 保留；未知 id 仍回退默认；`hold_key="none"` 保留且不要求 hold_combo；缺省字段仍落默认（既有用例保持绿）。
- GREEN：`Settings::sanitize` 音效循环加 `id == "none"` 保留分支；hold 分支在 custom 判定外增加 `self.hold_key == "none"` 合法保留。
- 验收：`cargo test -p voicekey settings` 相关用例通过。

<!-- comet-task-ref:ec7bcdfe-f2d3-4bc6-8c63-efb22ea0d044 -->
**1.2 hold_codes / hold_name 支持 `none`**（依赖：1.1）
- 现状核验：`hold_codes()` 对未知 id 返回空 Vec（已满足），`hold_name()` 返回空串（不满足）。
- RED：单测——`hold_key="none"` 时 `hold_codes()==[]`、`hold_name()=="未设置"`。
- GREEN：`hold_name()` 加 `none` 分支返回「未设置」。
- 验收：`cargo test -p voicekey` 通过。

<!-- comet-task-ref:79bb87dc-cef8-4e86-afb8-f3cb33918f5d -->
**1.3 sound 对 `none` 静音**（依赖：1.1）
- RED：`sound.rs` 测试——`gated_id` 对 `none` 返回 None；`play_duration` 返回 0（与总开关关闭同路径）。
- GREEN：预期无需改 `gated_id`（`is_known("none")` 恒 false）；若 RED 直接通过则记录为既有行为核验，不改源码。
- 验收：`cargo test -p voicekey sound` 通过。

<!-- comet-task-ref:f3653023-8d38-4c05-989c-84db1c224ddc -->
**1.4 hotkey hold=0 直通**（依赖：无）
- RED：`hotkey.rs` 测试——`Config::default()`（hold=0、无 chord）下修饰键按下/松开不产生 Press/LongPress/Release；tap 修饰键在阈值内点按仍产生 Tap。
- GREEN：预期现有实现满足（`e.code == cfg.hold` 在 hold=0 时恒 false）；RED 直接通过则记录为核验。
- 验收：`cargo test -p voicekey hotkey` 通过。

## 2. 后端：托盘展示

<!-- comet-task-ref:3eefc199-4d47-4524-ae5e-57abdbfd0427 -->
**2.1 托盘头部按状态组合**（依赖：1.2）
- 重构：把 `tray_menu` 头部文案提取为纯函数（如 `tray_head(hold: Option<&str>, tap: Option<&str>, paused: bool) -> String`），保持原渲染不变。
- RED：四种组合单测——hold 有/tap 有（现状文案）、hold 无/tap 有（只描述点按）、hold 有/tap 无（现状）、hold 无/tap 无（「语音输入」）；文案不含空按键名。
- GREEN：纯函数按组合生成；`tray_menu` 调用它（`paused` 分支维持「已暂停监听」）。
- 验收：`cargo test -p voicekey ui` 通过；`hold_name()` 返回「未设置」不出现在文案中。

## 3. 前端：设置页交互

<!-- comet-task-ref:c6e11042-0d0d-4ebe-9014-14c27f61e822 -->
**3.3 前端 store 测试**（依赖：无；先于 3.1/3.2 提供 RED）
- RED：`store.test.mjs` 补 `none` 用例——save({soundDone:"none"}) 后回读为 `none`；save({holdKey:"none"}) 同理。
- GREEN：store 的 save/merge 逻辑已满足则记录核验。
- 验收：`node --test app/src/lib/store.test.mjs` 通过。

<!-- comet-task-ref:46393672-651f-4a7a-8744-7968c1d503c7 -->
**3.1 Audio.svelte「无」选项**（依赖：1.1/1.3、3.3）
- 改动：四个事件下拉首项 `<option value="none">无</option>`；`{#if s[ev.key] !== "none"}` 包裹试听按钮。
- 验收：`npm --prefix app run build`（或 `npm run build`，以 app/package.json scripts 为准）通过；下拉含「无」。

<!-- comet-task-ref:4d5601c9-6587-4d2d-9712-e8dd8abf809a -->
**3.2 Keys.svelte 长按「无」**（依赖：1.1/1.2、3.3）
- 改动：长按下拉加 `<option value="none">无（停用长按）</option>`；`api.ts::holdName` 对 `none` 返回「未设置」；长按卡片复用未设置展示。
- 验收：构建通过；选「无」后卡片显示「未设置」。

## 4. 集成验证

<!-- comet-task-ref:ebf209f9-91bb-4d9f-8a84-8739fc9723af -->
**4.1 全量检查**（依赖：1–3 全部）
- `cargo test -p voicekey`、`cargo check`（workspace）、`npm run build`（app）。验收：全部退出码 0，用 `comet check run` 录制 build 证据。

<!-- comet-task-ref:5023f8a3-547f-49d0-9118-850cb8e95da9 -->
**4.2 验收场景对照**（依赖：4.1；粒度调整：交互式 GUI 走查需运行应用，移入 Workflow follow-up）
- 以自动化测试逐条对照 delta specs 场景并记录映射结果（逐事件置「无」静音/其余照常、sanitize 不回退、长按停用/点按正常、托盘文案如实、未选「无」行为不变）。

## 自检

- 任务与 tasks.md 一一对应（10 项），无第二套 checkbox。
- 无范围外任务；托盘纯函数提取属 2.1 范围内实现细节。
- TDD 顺序明确；预期"RED 直接通过"的核验类任务（1.3/1.4/3.3）如实记录，不伪造 RED。
- 回退：单 commit 系列，`git revert` 即可；配置中 `none` 由旧版回退默认，无数据损坏。
