# Design

## Context

简洁方案映射表 `SIMPLE_SCHEME` 同时驱动三处：`gated_id` 兜底取值、`simple_scheme_rows` 面板展示、`play_duration` 估算；系统音映射在 `simple_scheme_system_name`。面板简洁方案区块按事件序渲染，发声行带试听。

## Goals / Non-Goals

**Goals:** 简洁方案失败事件发声（macOS Basso / Windows 内置「警示」）；面板失败行展示音效名与试听。

**Non-Goals:** 不动「完成」事件（仍不发声）；不新增音效资源（复用池内 error.wav）；不改自定义方案。

## Decisions

1. **`SIMPLE_SCHEME` 追加 `(Error, "error")`**：一处改动同时让兜底取值、面板行、时长估算生效；`simple_scheme_system_name` 的 Error 分支返回 `Basso`（macOS 系统音），Done 保持 `None`。
2. **面板改为按固定事件序 + 映射查找渲染**：`{#each EVENTS as ev}`，`schemeRows.find` 命中则显示音效名+试听，未命中显示「不发声」；行序恒为开始/处理/完成/失败，不随后端映射表顺序漂移。原 `evInfo` 帮助函数不再使用，删除。
3. **试听即所得**：`play_id("error")` 在简洁方案下经既有映射播系统 Basso（macOS），无需新增试听逻辑。

## Risks / Trade-offs

- [失败音与 cometix-dictation 行为偏离] → 用户明确决策（选择 Basso）
- [`play_duration(Error)` 走 WAV 头估算而非 Basso 实际时长] → 当前无调用方（开始延迟用 Start、结果间隔用 Processing），不影响行为

## Open Questions

无。
