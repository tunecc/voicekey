# Design

## Context

自定义方案无实际使用；设置页双方案 + 逐事件控件构成不必要的复杂度。字段 `sound_scheme` 与逐事件字段（`sound_start` 等）保留在 Settings 中（serde 兼容旧配置），但恒按简洁方案处理。

## Goals / Non-Goals

**Goals:** 设置页只剩总开关 + 固定映射行；后端恒按简洁映射发声；两行描述去冲突。

**Non-Goals:** 不改提示音触发时机与音效映射本身；不删除 Settings 逐事件字段（仅忽略，避免 settings.json 结构变更）；不动 Onboard。

## Decisions

1. **后端恒简洁**：`sanitize()` 无条件 `sound_scheme = SCHEME_SIMPLE`（默认值同步改 simple）；`gated_id`/`play`/`play_id`/`play_duration` 删除 custom 分支，恒走 `SIMPLE_SCHEME` 映射；`SCHEME_CUSTOM` 常量与 `is_known_scheme` 移除。逐事件字段保留但不再被读取。
2. **前端无条件渲染简洁映射**：删除「音效方案」行与 custom 分支；`sounds`/`listSounds` 仅被自定义控件使用，一并移除。两行描述更新（见 proposal 文案）。
3. **测试同步**：settings 两个方案回退测试改为「恒简洁」断言；sound.rs 测试删除 custom 分支引用。

## Risks / Trade-offs

- [旧配置里 soundScheme:"custom" 与逐事件选择被静默忽略] → 唯一用户即维护者本机，且简洁映射即其日常实际行为；serde 忽略未知/多余字段，无兼容风险
- [Settings 保留死字段] → 换取 settings.json 零结构变更；后续清理另行处理

## Open Questions

无。
