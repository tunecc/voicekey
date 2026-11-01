# Proposal

## Why

自定义音效方案从未被实际使用（用户始终用简洁方案），却让设置页多出方案选择行与逐事件音效控件两层复杂度；且「识别处理」「识别完成」两行描述指向同一时刻、语义冲突（一个说结果就绪时响、一个说不发声）。收敛为唯一简洁方案并修正文案。

## What Changes

- 删除自定义方案：`sound.rs` 移除 `SCHEME_CUSTOM` 与逐事件分支（`gated_id`/`play`/`play_id`/`play_duration` 恒按简洁固定映射）；`settings.rs` 默认与 sanitize 强制简洁（旧 settings.json 里的 `soundScheme: "custom"` 及逐事件字段被忽略，serde 兼容）
- `Audio.svelte`：删除「音效方案」选择行与自定义方案逐事件控件分支；移除 `listSounds`/`sounds`（仅自定义控件使用）
- 文案修正：「识别处理」行描述改为「识别完成时轻啵一声」；「识别完成」行描述改为「识别很快，完成不另发音效」——两行不再冲突（轻啵即完成信号，完成不另发音）
- delta spec：audio-feedback「会话事件提示音」「提示音设置」「内置音效与跨平台一致」三条需求 MODIFIED（仅简洁方案；历史 scenario 名保留、内容重写为已移除语义）

## Capabilities

### New Capabilities

<!-- 无 -->

### Modified Capabilities

- `audio-feedback`: 移除自定义方案与方案选择；仅保留简洁固定映射；两行描述去冲突

## Impact

- `app/src-tauri/src/sound.rs`、`app/src-tauri/src/settings.rs`（含 2 个方案相关单测更新）、`app/src/pages/Audio.svelte`
- `docs/openspec/specs/audio-feedback/spec.md` 归档时合并
- 用户可感行为：设置页不再有音效方案选择与逐事件控件；提示音行为与此前简洁方案完全一致；曾配置自定义的旧配置回到简洁映射
