# Proposal

## Why

提示音目前是固定映射（开始=轻叮、处理=轻啵、失败=警示），用户希望四个事件都能从 **macOS 系统铃声**（/System/Library/Sounds 的 Tink/Pop/Basso/Glass 等）中自选，默认值保持现状语义：开始=Tink、处理=Pop、完成=不发声、失败=Basso（即面板当前的「警示」行音源，经实机确认接受其音量水平）。

## What Changes

- 设置字段复用：`sound_start/sound_processing/sound_done/sound_error` 四个字段改存 **macOS 系统音名**（或 `none`=不发声）；sanitize 校验值必须属于系统音集合 ∪ none，非法回退各事件默认
- 默认值：start=`Tink`、processing=`Pop`、done=`none`、error=`Basso`；旧配置中的池内音效 id（switch/minimize 等）不属于系统音集合 → sanitize 回退新默认
- `sound.rs`：新增 `SYSTEM_SOUNDS` 集合；播放改为按逐事件选中的系统音名 NSSound 直放（macOS）；Windows 无系统音，同名内置近似音（tink/pop/error.wav）兜底，其余选择在 Windows 静音（已知限制）
- `Audio.svelte`：四行各提供选择器（「不发声」+ 系统铃声列表）与试听（试听即所得）；移除固定映射展示（`simple_scheme`/`SIMPLE_SCHEME` 一并删除）
- `api.ts`/`main.rs`：`list_sounds` 改返回系统铃声列表；移除 `simple_scheme` 命令
- delta spec：audio-feedback 三条需求 MODIFIED

## Capabilities

### New Capabilities

<!-- 无 -->

### Modified Capabilities

- `audio-feedback`: 固定映射改为逐事件可选系统铃声；默认值如上；Windows 兜底限制记录

## Impact

- `app/src-tauri/src/sound.rs`、`app/src-tauri/src/settings.rs`、`app/src-tauri/src/main.rs`、`app/src/lib/api.ts`、`app/src/pages/Audio.svelte`
- 旧配置迁移：池内音效 id 回退新默认（无需迁移代码，sanitize 处理）
- 行为：默认听感与当前一致；用户可按事件换任意 macOS 系统铃声
