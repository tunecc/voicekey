# Design

## Context

提示音时机变更（processing-chime-after-restore）后面板文案未同步；简洁方案「识别完成 / 识别失败」合并行需拆分。事件描述表（Audio.svelte `EVENTS`）同时供简洁方案行与自定义方案逐事件行使用，一处修改两处生效。

## Goals / Non-Goals

**Goals:** 面板描述与实际行为一致；完成/失败分行展示。

**Non-Goals:** 不改提示音播放行为（sound.rs / controller.rs 不动）；不新增失败音（cometix-dictation 无失败音，保持一致）；不改自定义方案交互。

## Decisions

1. **只改展示层**：`EVENTS` 表中 `soundProcessing` 的描述改为「识别完成、音量恢复后（提示结果就绪）」；简洁方案区块把合并行拆为「识别完成」「识别失败」两行，行文案复用 `EVENTS` 的 label/desc（避免双源），各自显示「不发声」。
2. **失败音保持静音**：与 cometix-dictation 对齐（其失败仅 HUD 视觉提示）；如未来要加失败音另立 change。

## Risks / Trade-offs

- [识别处理与识别完成两行都指向结果就绪时刻，用户可能困惑] → 处理行描述明确「提示结果就绪」，完成行标注不发声，语义互补

## Open Questions

无。
