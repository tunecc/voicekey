---
comet_change: settings-none-options
role: technical-design
canonical_spec: openspec
archived-with: 2026-11-01-settings-none-options
status: final
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
