# Tasks

## 1. 后端：`none` 取值贯通（settings / sound / hotkey）

- [x] 1.1 `settings.rs::sanitize`：音效四字段 `id == "none"` 保留（其余未知 id 仍回退默认）；`hold_key == "none"` 合法保留、不要求 hold_combo。验证：`cargo test -p voicekey` 中新增 sanitize 用例（none 保留、未知 id 仍回退、legacy 缺省仍落默认）通过 <!-- comet-task:45612505-c891-42c4-8276-754a92b2342f -->
- [x] 1.2 `settings.rs`：`hold_codes()` 对 `none` 返回空 `Vec`，`hold_name()` 返回「未设置」。验证：单测断言 `hold_codes()==[]` 与 `hold_name()=="未设置"` <!-- comet-task:ec7bcdfe-f2d3-4bc6-8c63-efb22ea0d044 -->
- [x] 1.3 `sound.rs`：补 `gated_id`/`play_duration` 对 `none` 的用例（返回 None / 0 时长，同总开关关闭路径）。验证：`cargo test -p voicekey` 相关用例通过 <!-- comet-task:79bb87dc-cef8-4e86-afb8-f3cb33918f5d -->
- [x] 1.4 `hotkey.rs`：补 hold=0（空组合）用例——修饰键按下不产生 Press/LongPress，点按路径不受影响。验证：`cargo test -p voicekey` 热键用例通过 <!-- comet-task:f3653023-8d38-4c05-989c-84db1c224ddc -->

## 2. 后端：托盘与展示

- [x] 2.1 `ui.rs::tray_menu`：头部按 hold/tap 设置状态组合生成——长按停用只描述点按，两者皆未设置用「语音输入」。验证：四种组合（hold×tap 有无）单测通过，文案不出现空按键名 <!-- comet-task:3eefc199-4d47-4524-ae5e-57abdbfd0427 -->

## 3. 前端：设置页交互

- [x] 3.1 `Audio.svelte`：四个事件下拉加 `<option value="none">无</option>`（首项）；选「无」的行隐藏试听按钮。验证：`npm run build`（app 目录）通过；自定义方案下选「无」后该行无试听按钮 <!-- comet-task:46393672-651f-4a7a-8744-7968c1d503c7 -->
- [x] 3.2 `Keys.svelte` + `api.ts::holdName`：长按选择加「无（停用长按）」；`none` 时长按卡片展示「未设置」（复用点按未设置样式）。验证：`npm run build` 通过；选「无」后卡片显示未设置 <!-- comet-task:4d5601c9-6587-4d2d-9712-e8dd8abf809a -->
- [x] 3.3 前端测试：`store.test.mjs` 补 `none` 取值的 save/回显用例。验证：`node --test app/src/lib/store.test.mjs` 通过 <!-- comet-task:c6e11042-0d0d-4ebe-9014-14c27f61e822 -->

## 4. 集成验证

- [x] 4.1 全量检查：`cargo test -p voicekey` 与 `cargo check`（workspace）通过；`npm run build` 通过。验证：命令实际执行并记录结果 <!-- comet-task:ebf209f9-91bb-4d9f-8a84-8739fc9723af -->
- [x] 4.2 验收场景对照（粒度调整：交互式 GUI 走查需运行应用，本 change 环境无法执行，移入 Workflow follow-up；此处以自动化测试逐条对照 delta specs 场景记录结果）。验证：逐条映射 specs/audio-feedback 与 specs/hotkey-modes 场景到具体测试/验证结果并记录 <!-- comet-task:5023f8a3-547f-49d0-9118-850cb8e95da9 -->

## Workflow follow-up

- 交互式 GUI 手动走查（运行应用后）：自定义方案下把「识别完成」置「无」→ 完成静音、其余照常；重启后「无」不回退；长按置「无」→ 长按不触发、点按正常；托盘文案与设置页侧栏如实展示停用态。
- Verify 阶段完成唯一一次最终集成审查后，按归档流程确认归档与交付方式。
