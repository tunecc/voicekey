# Comet Design Handoff

- Change: settings-none-options
- Phase: design
- Mode: compact
- Context hash: d53c294d45ea63c591a7bbe100a7572f38e13b8fc5ba4ecc7b053e2ec6a64d61

Generated-by: comet-handoff.sh
Task hash policy: task-content-v1. Read tasks.md for live completion; excerpts are design-time context.

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## docs/openspec/changes/settings-none-options/proposal.md

- Source: docs/openspec/changes/settings-none-options/proposal.md
- Lines: 1-29
- SHA256: 287babe8b64b0c4f98f10e47bc5433b3e65abe73618ddc6206c81ed63e7e221b

```md
# Proposal

## Why

设置中凡是用户可自定义的选择项，都应能回到「不使用」的状态，自定义才完整。当前自定义音效方案的四个事件只能从内置音效中选一个，长按说话必须绑定一个按键——用户既不能单独关掉某个事件的提示音，也不能停用长按模式。

## What Changes

- 自定义音效方案下，录音开始 / 识别处理 / 识别完成 / 识别失败四个事件的下拉增加「无」选项：选择后该事件不再发声，其余事件不受影响。
- 「无」是显式且可持久化的取值：选择后重启应用，配置 sanitize 不得把它回退成默认音效。
- 长按说话的按键选择增加「无」：选择后停用长按触发（点按说话保持原行为），设置页与托盘菜单对「未设置」有明确展示。
- 未选择「无」时所有行为与现状一致：默认值不变，提示音总开关与音效方案语义不变。

## Capabilities

### Modified Capabilities

- `audio-feedback`：自定义方案的逐事件音效选择控件必须提供「无」；选中「无」的事件不发声，且该选择随设置持久化。

### New Capabilities

- `hotkey-modes`：长按说话与点按说话两种触发模式可独立启用；长按说话可置「无」停用，停用后点按触发不受影响。

## Impact

- 前端：`app/src/pages/Audio.svelte`（音效下拉加「无」）、`app/src/pages/Keys.svelte`（长按键下拉加「无」与未设置展示）。
- 后端：`app/src-tauri/src/settings.rs`（sanitize 与 `hold_codes`/`hold_name` 支持「无」）、`app/src-tauri/src/sound.rs`（gated_id 对「无」不发声）、`app/src-tauri/src/hotkey.rs`（`Config::set_hold(&[])` 已支持空组合，核对触发链路）。
- 托盘：`app/src-tauri/src/ui.rs` 菜单头对未设置长按键的展示。
- 不修改提示音总开关、音效方案的语义；不改动点按快捷键与麦克风已有的清除/默认项。

```

## docs/openspec/changes/settings-none-options/design.md

- Source: docs/openspec/changes/settings-none-options/design.md
- Lines: 1-67
- SHA256: 154d082d95a0834477c0199c17bcca5fd341fc45d14b63ec0096cb6a477a3455

```md
---
comet_change: settings-none-options
role: technical-design
canonical_spec: openspec
---

# Design

## Context

设置的后端单一来源是 `app/src-tauri/src/settings.rs`（`Settings` + `sanitize`），提示音播放门控在 `sound.rs::gated_id`，长按键经 `settings.rs::hold_codes()` 进入 `hotkey.rs::Config::set_hold`（已支持空组合 = 长按停用）。前端设置页为 Svelte：`Audio.svelte`（音效下拉）、`Keys.svelte`（长按/点按卡片）；托盘文案在 `ui.rs::tray_menu`。现状：音效字段存音效 id（未知 id 会被 sanitize 回退默认），`hold_key` 取预设 id 或 `custom`。动机见 proposal.md。

## Goals / Non-Goals

**Goals:**

- 为「无」定义统一、可持久化的取值，贯通 前端下拉 → settings.json → sanitize → 播放/热键 门控。
- 长按停用复用 `set_hold(&[])` 既有路径，不新增热键状态机分支。
- 未选「无」时行为与现状逐字节一致。

**Non-Goals:**

- 不改提示音总开关、音效方案（simple/custom）的语义。
- 不动点按快捷键（已有清除）与麦克风（已有系统默认）的现有中性项。
- 不处理旧版本应用读取新配置的迁移（见 Risks）。

## Decisions

### D1：「无」统一用保留字 `none`

四个音效字段（`sound_start/processing/done/error`）与 `hold_key` 的新合法取值统一为字符串 `"none"`。

- 备选：音效用空字符串 `""`。否决：与「字段缺失」难区分（序列化后不可见）、与 `hold_key` 的 `custom` 保留字风格不一致。
- 兼容性：结构体 `#[serde(default)]` 下缺字段仍落到 `Default`（默认音效/默认按键），显式 `none` 才表示停用；旧配置中的未知 id 仍回退默认，行为不变。
- `gated_id`：`is_known("none")` 为 false → 返回 None → 不发声，现有逻辑天然兼容，仅需 sanitize 保留 `none`。

### D2：sanitize 规则扩展

`settings.rs::sanitize`：

- 音效字段：`id == "none"` 保留；其余未知 id 仍回退各自默认（现状不变）。
- `hold_key == "none"` 合法保留（不要求 `hold_combo`）；`custom` 与预设键分支不变；未知值仍回退第一个预设键。
- `hold_codes()`：`none` → 空 `Vec`；`hold_name()`：`none` → `"未设置"`。

### D3：热键与托盘

- 热键：`set_hold(&[])` 置 `hold=0, chord=None`；`tap` 判定独立于 `hold`，停用长按不影响点按。`handle()` 中 `e.code == cfg.hold` 在 hold=0 时恒 false（键码不含 0），无长按计时分支会被触达。
- 托盘头部（`ui.rs::tray_menu`）：按 hold/tap 是否设置组合生成——长按停用时只描述点按；两者都未设置时用中性文案「语音输入」。不出现在文案中的是停用侧，避免空按键名。

### D4：前端交互

- `Audio.svelte`：每个事件下拉首项加 `<option value="none">无</option>`；选「无」的行隐藏试听按钮（与试听要求「不发声的事件不提供试听」一致）。
- `Keys.svelte`：长按选择加「无（停用长按）」；`holdName()`（api.ts）对 `none` 返回「未设置」，长按卡片复用点按卡片已有的未设置展示样式。

## Risks / Trade-offs

- [旧版本应用读到 `none` 会按未知值回退默认（恢复发声/恢复默认按键）] → 只影响混用新旧版本同一配置的场景，不损坏配置；跨版本配置迁移不在本次范围。
- [`gated_id`/`play_duration` 对 `none` 的时长估算] → `none` 返回 0 时长，压音量延迟按 0 处理（与总开关关闭同路径），补测试锁定。
- [托盘文案组合分支增多] → `hold none/tap 有无` 四种组合补单元测试。

## Migration Plan

无数据迁移：`none` 是新增合法取值，旧配置缺省仍走默认。回滚 = 还原代码；配置中残留的 `none` 由旧版 sanitize 回退默认，无损坏。

## Open Questions

无。

```

## docs/openspec/changes/settings-none-options/tasks.md

- Source: docs/openspec/changes/settings-none-options/tasks.md
- Lines: 1-28
- SHA256: 3a075097cc6fb880f1a6c1cee65e0ca334c9df3c6a5856b3f0e0fc8e908c6b36

```md
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

```

## docs/openspec/changes/settings-none-options/.openspec.yaml

- Source: docs/openspec/changes/settings-none-options/.openspec.yaml
- Lines: 1-2
- SHA256: 108f7b3dae33488206672f31b510275110c0194c93062eb1970e19ca60c08722

```md
schema: spec-driven
created: 2026-11-01

```

## docs/openspec/changes/settings-none-options/specs/audio-feedback/spec.md

- Source: docs/openspec/changes/settings-none-options/specs/audio-feedback/spec.md
- Lines: 1-91
- SHA256: 2b0093d2ead02087b49a35944d8f088054d89cd43e31755ef508837eb362d9b8

[TRUNCATED]

```md
# Spec Delta

## MODIFIED Requirements

### Requirement: 会话事件提示音

应用必须在录音会话的四个关键事件（录音开始、进入识别处理、处理完成（结果就绪）、识别失败（含转写超时与出错））遵循用户选择的音效方案与总开关发出提示音：简洁方案仅开始与处理发声（纯双音，见下述场景），完成与失败不发声；自定义方案下各事件按逐事件选择发声，选择「无」的事件不发声，其余事件互不干扰。取消录音时不得播放任何提示音。

#### Scenario: 录音开始发声

- **WHEN** 用户通过热键或托盘触发一次正式录音开始
- **THEN** 播放「录音开始」事件在当前音效方案下使用的提示音

#### Scenario: 进入识别处理发声

- **WHEN** 录音结束、应用进入识别处理状态
- **THEN** 播放「处理」事件在当前音效方案下使用的提示音

#### Scenario: 处理完成发声

- **WHEN** 识别结果就绪（单候选直接注入，或多候选面板呈现）
- **THEN** 播放「完成」事件在当前音效方案下使用的提示音

#### Scenario: 识别失败发声

- **WHEN** 识别失败、出错或转写超时
- **THEN** 播放「失败」事件在当前音效方案下使用的提示音

#### Scenario: 简洁方案开始与处理按固定映射发声

- **WHEN** 音效方案为简洁，录音开始或进入识别处理
- **THEN** 录音开始播轻巧"叮"声、进入识别处理播轻弹"啵"声；macOS 上播放系统提示音 Tink 与 Pop（与 cometix-dictation 一致），Windows 上播放内置近似音；系统音不可用时回退内置近似音

#### Scenario: 简洁方案完成与失败不发声

- **WHEN** 音效方案为简洁，识别完成或识别失败
- **THEN** 不播放提示音（纯双音极简风格）

#### Scenario: 自定义方案逐事件发声

- **WHEN** 音效方案为自定义
- **THEN** 各事件播放用户为该事件选中的音效；选择为「无」的事件不发声，四个事件的发声互不干扰

#### Scenario: 取消录音不发声

- **WHEN** 用户在识别完成前取消录音会话
- **THEN** 不播放任何提示音

### Requirement: 提示音设置

音频设置页必须提供提示音设置组：音效方案选择控件（简洁、自定义两选一，默认自定义），以及提示音总开关（默认开启）。自定义方案下必须提供录音开始、处理、完成、失败四个事件各自独立的音效选择控件，每个控件必须在「无」与内置音效集合之间选择，「无」表示该事件不发声；简洁方案下必须以只读方式展示发声事件（录音开始、进入识别处理）的固定音效映射，并明确标注完成与失败不发声，且不得提供逐事件音效选择控件。音效方案、总开关与逐事件选择（含「无」）的任一设置变更必须即时生效并持久化。

#### Scenario: 关闭总开关后全静音

- **WHEN** 提示音总开关处于关闭状态
- **THEN** 无论音效方案为何，四个事件均不播放提示音，其余功能行为不变

#### Scenario: 方案切换即时生效

- **WHEN** 用户在简洁与自定义之间切换音效方案
- **THEN** 后续会话事件立即按新方案的规则发声，无需重启应用

#### Scenario: 简洁方案隐藏逐事件选择

- **WHEN** 音效方案为简洁
- **THEN** 设置页只读展示开始与处理的固定音效映射，标注完成与失败不发声，不展示逐事件音效选择控件

#### Scenario: 自定义方案恢复逐事件选择

- **WHEN** 音效方案切回自定义
- **THEN** 四个事件的音效选择控件恢复显示，且各事件保持此前已保存的选择（含「无」）

#### Scenario: 逐事件选择独立生效

- **WHEN** 音效方案为自定义，用户为某一个事件选择不同的音效或「无」
- **THEN** 该事件按新选择发声或不发声，其余事件保持各自原有音效

#### Scenario: 逐事件「无」持久化

- **WHEN** 用户为某个事件选择「无」后重启应用

```

Full source: docs/openspec/changes/settings-none-options/specs/audio-feedback/spec.md

## docs/openspec/changes/settings-none-options/specs/hotkey-modes/spec.md

- Source: docs/openspec/changes/settings-none-options/specs/hotkey-modes/spec.md
- Lines: 1-31
- SHA256: ae69ab02840ac690484c263874e7fbd960adaa7f76cdfdf0c7864487dd0993a9

```md
# Spec Delta

## Purpose

长按说话与点按说话是两种可独立使用的语音触发模式；本能力规定两种模式的启用组合与停用行为，使用户可以只保留自己需要的触发方式。

## ADDED Requirements

### Requirement: 长按说话可停用

快捷键设置页的长按说话按键选择必须提供「无」选项；选择「无」后长按触发整体停用——按住任何修饰键或组合键都不得开始录音，点按说话的行为不受影响。「无」选择必须随设置持久化；快捷键设置页与托盘菜单必须如实展示停用状态，不得显示空的按键名或误导性文案。用户选择具体按键或自定义组合时，长按行为与既有行为一致。

#### Scenario: 置「无」后长按不触发

- **WHEN** 用户把长按说话设为「无」
- **THEN** 此后按住任何修饰键或组合键都不再开始录音；点按快捷键的录音流程保持原行为

#### Scenario: 停用状态持久化

- **WHEN** 长按说话处于「无」状态时重启应用
- **THEN** 该选择保持，长按仍处于停用状态，点按说话正常可用

#### Scenario: 停用状态有明确展示

- **WHEN** 长按说话为「无」
- **THEN** 快捷键设置页的长按卡片展示未设置状态；托盘菜单头部对触发方式的描述如实反映停用，不出现空按键名

#### Scenario: 从「无」改回具体按键即时生效

- **WHEN** 用户把长按说话从「无」改选为某个预设键或自定义组合
- **THEN** 长按触发立即恢复，行为与停用前一致，点按说话不受影响

```
