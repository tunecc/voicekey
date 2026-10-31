---
comet_change: simple-sound-scheme
role: technical-design
canonical_spec: openspec
archived-with: 2026-10-31-simple-sound-scheme
status: final
---

# Design

## Context

voicekey 桌面应用（`app/`，Tauri 2：Rust 后端 + Svelte 5 前端）已有一套逐事件可配置的多音效机制（见 `docs/openspec/changes/archive/2026-10-31-audio-feedback-sounds/design.md`，其 D1–D7 决策继续有效）：`sound.rs` 以 rodio 播放 `resources/sounds/` 下的内置 WAV（现 6 个，Kenney Interface Sounds，CC0），四个会话事件各自从池中选音效，`settings.rs` 扁平 camelCase 字段持久化，`Audio.svelte` 提供总开关 + 逐事件下拉 + 试听。事件触发点集中在 `controller.rs` 状态机，经 `sound::play(&AppHandle, SoundEvent)` 收口。动机见 proposal.md，行为要求见 specs/audio-feedback/spec.md delta。

约束（2026-10-31 走查后修订）：自定义方案的音效选择不得依赖操作系统音效目录，macOS 与 Windows 的可选集合一致；简洁方案经用户确认改为纯双音复刻 cometix-dictation——macOS 直接运行时引用系统音 Tink/Pop（不随应用分发，规避再分发问题），Windows 用内置 CC0 近似音兜底（见 specs delta「内置音效与跨平台一致」）。

## Goals / Non-Goals

**Goals:**

- 新增「简洁」方案：纯双音复刻 cometix-dictation（开始=Tink、处理=Pop，完成/失败不发声），macOS 直接播放系统音获得与 cometix 完全一致的听感
- 方案切换即时生效并持久化；简洁与自定义（现有多音效）真实并存
- 新素材统一进入内置音效池（Windows 简洁方案兜底音与自定义方案共用）

**Non-Goals:**

- 不把 Apple 系统音文件打包进应用（不可再分发）；macOS 简洁方案经 NSSound 运行时引用系统音，Windows 用内置 CC0 兜底
- 不删除或替换现有 6 个音效；不改变事件触发点、播放引擎与主流程时序
- 不做按方案独立音量、自定义音效导入、随系统主题切换音色

## Decisions

### D1：方案门控收口在 Rust 侧 `gated_id`，只新增一个设置字段

`Settings` 新增 `sound_scheme: String`（`"simple" | "custom"`，默认 `"custom"`）。`sound.rs` 在现有 `gated_id()` 中加一个分支：`custom` 沿用逐事件字段；`simple` 忽略逐事件字段，改查固定映射常量表（见 D2）。理由：所有事件发声都已从 `sound::play` 收口，单点分支改动最小、两端（触发与试听）天然一致。备选（已否决）：把方案实现为「前端预设按钮，一键改写四个逐事件字段」——不是真正的模式并存，切回自定义会丢失用户原有选择，且无法满足规格「简洁方案下不得提供逐事件选择控件、切回后恢复此前选择」。

- `sanitize()` 校验 `sound_scheme`：非合法值回退 `"custom"`；旧配置缺字段由 `#[serde(default)]` 兜底，现有用户行为不变。
- 切换方案**不得清除**逐事件字段——切回自定义时恢复此前选择（spec 场景要求）。

### D2：简洁方案的固定映射与音效素材（走查后修订：纯双音复刻）

用户实机走查后明确：CC0 近似音与 cometix-dictation 的实际听感不同，要求**纯双音复刻**——简洁方案只有两个发声事件：开始=**Tink**、处理=**Pop**；完成与失败**不发声**（cometix 本就没有对应音，用户在三个选项中明确选择纯双音）。

- **macOS 直接播放系统音 Tink/Pop**（`NSSound(named:)`，与 cometix-dictation 同一 API、同一声音，运行时引用系统目录、不随应用分发，规避再分发问题）。Windows 无对应系统音，回退播放内置 CC0 近似音 `tink.wav`/`pop.wav`（此前挑选的 tick_004/drop_003 转码件）。
- 原挑选的 `ding`（轻铃）/`soft-alert`（低鸣）保留在内置音效池（自定义方案仍可选），不再被简洁方案引用；LICENSE.md 用途标注同步更新。
- 备选（已否决）：继续在 CC0 素材中找更近似音（听感仍不可能与 Apple 系统音一致，用户已否定）。

### D3：新增 Tauri 命令 `simple_scheme()` 供前端展示映射

`main.rs` 注册 `simple_scheme() -> Vec<{event, soundId, soundName}>`，由 Rust 常量表生成（修订：仅包含发声的开始/处理两行；完成/失败不发声由前端在映射区尾部以只读说明行标注，避免「映射行播放 undefined」一类歧义）。理由：前端不硬编码映射，Rust 常量表保持单一来源（与 `list_sounds()` 同一模式）。

### D4：Audio.svelte 增加方案单选，条件渲染两种配置形态

提示音设置组顶部新增「音效方案」行：简洁/自定义二选一（分段按钮样式），变更经 `save({ soundScheme })` 即时持久化。`soundScheme === "simple"` 时隐藏四行逐事件 `select`，渲染 D3 的只读映射行（开始/处理两行，事件名 + 音效名 + 试听按钮）与「完成与失败不发声」说明行；`"custom"` 时恢复现有逐事件行（值来自既有设置字段，不清空）。总开关行位置与行为不变。

### D8：macOS 系统音播放机制（走查后新增）

- **播放路径**：简洁方案在 macOS 上经 `objc2-app-kit` 的 `NSSound` 播放系统音（`soundNamed:` + `play`）。调用统一投递到主线程（`AppHandle::run_on_main_thread`，与 cometix-dictation 的 `DispatchQueue.main.async` 同策略）。`objc2-app-kit` 已在依赖树（tauri→muda），仅需启用 `NSSound` 特性，无新增编译负担。
- **回退链**：macOS 系统音不可用（`soundNamed:` 返回 nil，仅系统损坏时出现）或非 macOS 平台 → 回退播放内置 CC0 近似音（既有 rodio 路径）；播放失败仍静默降级（既有惯例）。
- **试听即所得**：简洁方案行内试听复用同一发声源解析——当前方案为简洁且 id 属于简洁映射（tink/pop）时播系统音，否则按池内 id 播 WAV。自定义方案试听行为不变。
- 备选（已否决）：`afplay` 子进程——无需 unsafe 但进程启动延迟明显，听感与 cometix 不一致。

## Risks / Trade-offs

- [macOS 简洁方案依赖系统音目录（偏离"全内置"初版规格）] → 用户走查后明确选择纯双音复刻并知晓取舍；系统音由 NSSound 运行时定位、不随应用分发，无版权问题；Windows 与系统音不可用时回退内置近似音，发声语义不缺失
- [ objc2 调用引入 unsafe] → 仅限 NSSound 两个方法的窄封装，主线程投递复用 Tauri `run_on_main_thread`；独立审查覆盖
- [挑选的 CC0 兜底音与 Tink/Pop 听感有差距] → 仅影响 Windows 与系统音异常场景；换 WAV 不动代码（id 稳定）
- [简洁映射 id 将来调整导致旧构建配置引用失效] → 映射是常量、不写入用户配置；逐事件字段 sanitize 兜底，不影响
- [方案字段被旧版前端读到未知值] → serde default + sanitize 回退 `custom`，后端始终权威
- [新素材授权瑕疵] → 只取 CC0/公有领域来源，LICENSE.md 逐文件登记出处；与现有素材同标准

## Migration Plan

无数据迁移。旧配置缺 `sound_scheme` 字段 → serde 默认 `custom`，行为与升级前一致；回滚即移除新字段与新增资源，无残留状态。

## Open Questions

无。具体选中哪个音频文件属构建期试听细节，不改变规格与任务拆分。
