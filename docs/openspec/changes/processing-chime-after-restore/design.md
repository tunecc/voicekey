# Design

## Context

「处理」提示音当前在 `controller.rs` 录音结束进入识别的路径播放（`SoundEvent::Processing`），该时刻音量 ducking 仍处于压低态（还原为 worker 内异步淡入 0.25–0.45s，无完成回调）。规格与现状见 proposal。

## Goals / Non-Goals

**Goals:** 处理提示音在结果就绪、音量恢复后以全音量播放；不阻塞识别与结果注入。

**Non-Goals:** 不改事件配置模型与持久化；不新增平台音量 API（升级判定约束）；不调整失败音的播放时机。

## Decisions

1. **结果就绪路径统一改为「处理→完成」序列**：`controller.rs` 中所有识别结果就绪的 `SoundEvent::Done` 播放点，改为先播 `Processing` 再播 `Done`（两音间隔取「处理」音的 `play_duration`，避免重叠）；录音结束路径的 `Processing` 播放删除。失败路径只保留 `Error`，不播「处理」。
2. **恢复等待用本地时间估算，不动平台 crate**：`Ctl` 记录 `stopped_at: Instant`（`stop_recording` 时写入）；结果就绪时 `等待 = 恢复窗口上限(450ms，与平台 FADE_MAX 对齐) − 距结束时长`，为 0 则立即播。识别延迟通常远大于 450ms，该等待实际几乎不触发；极端快速识别下以窗口上限兜底，不追求精确对齐淡出曲线。
3. **异步播放不阻塞注入**：等待与播放都在 `tauri::async_runtime::spawn` 中执行，与现有提示音播放一致，满足「主流程不受影响」需求。

## Risks / Trade-offs

- [自定义方案结果就绪时连播两声] → 用户已确认接受；两音按时长间隔避免重叠
- [极端快速识别（<450ms）时等待按窗口上限兜底，可能与淡出尾段轻微重叠] → 概率极低且听感为音量渐满，接受
- [多候选面板路径的「处理」音在面板呈现时播] → 与单候选路径语义一致（结果就绪）

## Open Questions

无。
