# Proposal

## Why

简洁方案目前识别失败完全静音（复刻 cometix-dictation），失败时只有 HUD 视觉提示，容易错过。用户选定失败音效：macOS 播系统音 **Basso**（经典错误音），Windows 回退内置「警示」（error.wav）——与现有 Tink/Pop 的系统音+兜底模式完全一致。

## What Changes

- `sound.rs`：简洁方案固定映射 `SIMPLE_SCHEME` 增加失败事件（→ 内置「警示」兜底）；`simple_scheme_system_name` 失败事件映射 macOS 系统音 `Basso`；「完成」仍不发声
- `Audio.svelte`：简洁方案区块改为按固定事件序渲染（发声行带音效名+试听，未映射事件显示「不发声」），「识别失败」行自动获得「警示」与试听按钮；试听即所得（play_id 已走系统音映射）
- delta spec：audio-feedback 三条需求 MODIFIED（会话事件提示音、提示音设置、内置音效与跨平台一致）；失败发声是对 cometix-dictation 行为的主动偏离（用户决策）

## Capabilities

### New Capabilities

<!-- 无 -->

### Modified Capabilities

- `audio-feedback`: 简洁方案失败事件从静音改为发声（macOS Basso / Windows 内置警示）；面板失败行展示音效名与试听

## Impact

- `app/src-tauri/src/sound.rs`、`app/src/pages/Audio.svelte`
- `docs/openspec/specs/audio-feedback/spec.md` 归档时合并
- 用户可感行为：简洁方案识别失败时播 Basso（此前静音）；设置页识别失败行出现「警示」与试听按钮
