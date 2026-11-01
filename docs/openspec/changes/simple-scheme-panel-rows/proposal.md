# Proposal

## Why

上一变更（processing-chime-after-restore）把「处理」提示音的播放时机移到了「识别结果就绪、系统输出音量恢复后」，但设置面板未同步：简洁方案下「识别处理」行描述仍写「录音结束进入识别时」（与实际行为矛盾）；且「识别完成 / 识别失败」挤在一行展示，看不出各阶段语义。经查参考项目 cometix-dictation 无失败音（仅 Tink/Pop 两声），本 fork 简洁方案保持失败静音与其一致。

## What Changes

- `Audio.svelte` 事件描述表：「识别处理」描述从「录音结束进入识别时」改为「识别完成、音量恢复后（提示结果就绪）」——同时作用于简洁方案行与自定义方案逐事件行（两方案的处理音时机一致）
- 简洁方案区块：「识别完成 / 识别失败」合并行拆为「识别完成」「识别失败」两行，各自标注不发声（行为不变，仅展示拆分）
- delta spec：audio-feedback「提示音设置」需求中简洁方案展示条款相应更新；提示音行为本身不变

## Capabilities

### New Capabilities

<!-- 无 -->

### Modified Capabilities

- `audio-feedback`: 「提示音设置」需求——简洁方案只读展示条款：处理行描述体现新时机，完成与失败分行展示

## Impact

- `app/src/pages/Audio.svelte`（仅展示层）
- `docs/openspec/specs/audio-feedback/spec.md` 归档时合并
- 行为不变：简洁方案仍为开始「叮」、结果就绪「啵」（音量恢复后）、失败静音
