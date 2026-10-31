# Proposal

## Why

录音会话提示音目前只提供「逐事件自选」的多音效配置（4 个事件 × 6 音效池），用户反馈配置太繁杂、整体听感偏吵。同类工具 cometix-dictation 的提示音体验更克制：仅在录音开始播一声轻巧的 Tink、结束播一声轻弹的 Pop，一个总开关即可，无逐项配置。voicekey 需要一个开箱即用的「简洁」方案提供同等克制的听感，同时保留现有多音效方案供需要区分各阶段的用户使用。

## What Changes

- 新增「音效方案」设置项：**简洁**（固定发声映射，不可逐事件更改）与**自定义**（现有多音效逐事件选择）二选一；默认值为自定义，现有用户行为不变
- 简洁方案固定映射（复刻 cometix-dictation 的极简风格，并把 voicekey 独有的事件补齐）：
  - 录音开始 → tink 类轻巧短音（对应 cometix 的 Tink）
  - 进入识别处理 → pop 类轻弹音（对应 cometix 停止录音时的 Pop）
  - 识别完成 → 轻快确认音（上网检索挑选）
  - 识别失败 → 低调不刺耳的短音（上网检索挑选）
- 上网检索并新增若干 CC0/可再分发音效素材：优先挑选音色特征贴近 Tink/Pop、与现有音效池风格协调、不突兀的短音；新素材统一进入内置音效池，多音效方案的逐事件下拉同步可选
- 音频设置页新增方案单选：选择简洁时隐藏逐事件音效选择行；选择自定义时恢复现有 UI；切换即时生效并持久化
- 不改变：播放引擎（rodio 常驻输出流）、触发点与会话事件映射、取消录音不发声、播放失败静默降级、总开关行为

## Capabilities

### New Capabilities

（无）

### Modified Capabilities

- `audio-feedback`: 提示音设置需求新增音效方案选择（简洁/自定义）；会话事件提示音需求新增简洁方案的固定发声映射与逐事件选择仅在自定义方案生效的约束；内置音效与跨平台一致需求随音效池扩充保持不变含义

## Impact

- **代码**：`app/src-tauri/src/sound.rs`（方案门控、音效池扩充）、`app/src-tauri/src/settings.rs`（新增方案字段）、`app/src/lib/api.ts`（设置字段镜像）、`app/src/pages/Audio.svelte`（方案单选 UI）
- **资源**：`app/src-tauri/resources/sounds/` 新增音效文件并更新 `LICENSE.md`（来源与授权清单）
- **规格**：`docs/openspec/specs/audio-feedback/spec.md` 随归档同步
- **不涉及**：遗留 Swift 应用与 iOS/Android 客户端、识别与注入主流程、新增 Rust 依赖
