# Design

## Context

固定映射（SIMPLE_SCHEME）限制四个事件的音效；用户需要逐事件从 macOS 系统铃声中选择。现有逐事件字段（sound_start 等）恰好可复用为系统音名存储，避免 settings.json 结构变更。

## Goals / Non-Goals

**Goals:** 四事件各自可选 macOS 系统铃声或「不发声」；默认 start=Tink、processing=Pop、done=none、error=Basso；试听即所得。

**Non-Goals:** 不提供自定义音源导入；不为 Windows 移植系统音（同名近似音 tink/pop/error.wav 兜底，其余选择 Windows 静音，已知限制）。

## Decisions

1. **字段复用 + 值域切换**：四个逐事件字段改存系统音名；`SYSTEM_SOUNDS` 常量列出 /System/Library/Sounds 标准集合（14 个）；`is_known` 改查该集合；`none` 表示不发声（沿用 settings::NONE）。旧池内 id 不在集合中 → sanitize 回退各事件默认。
2. **播放路径统一**：`play(event)` 取逐事件值 → macOS 用 `NSSound(named:)` 主线程直放；Windows 查同名内置 wav（tink/pop/error 存在）作兜底，无同名则静音。`simple_scheme_system_name`/`SIMPLE_SCHEME`/`simple_scheme_rows` 删除；`play_id`（试听）校验系统音名后直放。
3. **UI 回归逐事件选择器**：四行 = 描述 + 选择器（「不发声」+ 系统铃声）+ 试听；`list_sounds` 改返回系统铃声（id=name=英文名）；`simple_scheme` 命令移除。

3. **提示音会话代数门闩**：快速连续开始/停止时，延迟提示音（处理音/失败音的恢复等待）会跨会话堆积错拍。`Ctl.chime_gen`（AtomicU64）在每次开始录音时递增；延迟任务到期发现代数变化即丢弃。用户实测反馈的「连续点击出现奇怪声音」由此消除。

## Risks / Trade-offs

- [spec 一天内第三次调整（固定映射→逐事件）] → 用户驱动的迭代；delta spec 覆盖
- [Windows 上非默认选择静音] → 已知限制，记录于 spec；默认值跨平台均可发声
- [旧配置池内 id 失效] → sanitize 回退默认，无启动风险

## Open Questions

无。
