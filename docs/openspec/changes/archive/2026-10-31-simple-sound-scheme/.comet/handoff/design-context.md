# Comet Design Handoff

- Change: simple-sound-scheme
- Phase: design
- Mode: compact
- Context hash: dc27c991b91ba6f92ded48eb00ca770477b54da3aca6ad7ac89f53882fa46d1d

Generated-by: comet-handoff.sh
Task hash policy: task-content-v1. Read tasks.md for live completion; excerpts are design-time context.

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## docs/openspec/changes/simple-sound-scheme/proposal.md

- Source: docs/openspec/changes/simple-sound-scheme/proposal.md
- Lines: 1-34
- SHA256: dddb2e1113d74faad28fe841523f9116907b28ce054bb3a05a1fe398905e0862

```md
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

```

## docs/openspec/changes/simple-sound-scheme/design.md

- Source: docs/openspec/changes/simple-sound-scheme/design.md
- Lines: 1-76
- SHA256: 6264f2f2d67a251dfff5292f922472ea0ed9e1d77fdba6ea70f8acc434a4b3e1

```md
---
comet_change: simple-sound-scheme
role: technical-design
canonical_spec: openspec
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

```

## docs/openspec/changes/simple-sound-scheme/tasks.md

- Source: docs/openspec/changes/simple-sound-scheme/tasks.md
- Lines: 1-39
- SHA256: c94a397529009caa595ab8b3fedc03cc7fcec232f0c7251bfd10e9b873712bf5

```md
# Tasks

## 1. 音效素材检索与入库

- [x] 1.1 上网检索并挑选 4 个 CC0/可再分发音效：tink 类轻巧短音（开始）、pop 类轻弹音（处理）、轻快确认音（完成）、低调柔和短音（失败），时长 ≤0.3s、与现有音色协调不突兀；记录每个候选的来源页与授权 — 验证：4 个原始音频文件到手，来源与授权信息可追溯 <!-- comet-task:ec7930d8-6234-4370-a4d7-66ee14299f80 -->
- [x] 1.2 按现有规范（ffmpeg 裁首尾静音、mono、44.1kHz/16-bit PCM WAV）转码并以稳定 id 命名放入 `app/src-tauri/resources/sounds/`，在 `LICENSE.md` 增补来源、原始文件名、时长与授权清单 — 验证：`file` 确认 WAV 格式与参数，LICENSE.md 含全部新条目 <!-- comet-task:03fa2e7e-5fd3-48d6-a7a3-b296bbf0cdf5 -->
- [x] 1.3 `sound.rs` 的 `SOUNDS` 池登记新音效（id、中文名） — 验证：`cargo test sound_pool_is_not_empty_and_ids_match_wav_files` 通过 <!-- comet-task:120a36dc-fc9a-4525-99c4-6015b6a46af3 -->

## 2. Rust 侧方案门控

- [x] 2.1 `settings.rs` 新增 `sound_scheme` 字段（默认 `"custom"`、serde default 兜底、sanitize 校验非法值回退），`app/src/lib/api.ts` 镜像字段 — 验证：`cargo test` 新增用例通过（缺字段兜底、非法值回退） <!-- comet-task:35e24cc4-c508-4ece-8632-c3748bb2b0a1 -->
- [x] 2.2 `sound.rs` 新增简洁方案固定映射常量表与 `gated_id` 门控分支：`simple` 按映射取音效 id，`custom` 沿用逐事件字段，总开关逻辑不变 — 验证：`cargo test` 新增用例通过（simple 四事件按映射发声、custom 行为不变、总开关关闭全静音） <!-- comet-task:f0265e22-2f91-4972-b391-2605efe0eec5 -->
- [x] 2.3 确认方案切换不写逐事件字段：映射为常量读取，设置保存路径仅新增 scheme 字段 — 验证：单测断言 simple 门控只读不改写四个逐事件字段 <!-- comet-task:1558469e-773d-4b92-80b0-bcf9d2314113 -->

## 3. Tauri 命令

- [x] 3.1 `main.rs` 新增并注册 `simple_scheme()` 命令，返回四事件固定映射（事件键、事件标签、soundId、soundLabel），数据由 Rust 常量表生成 — 验证：调用命令返回正确映射，且每个 soundId 均在 `SOUNDS` 池内 <!-- comet-task:0a777fbe-8264-446b-ab92-c8b76e811e42 -->

## 4. 前端设置页

- [x] 4.1 `api.ts` 新增 `simple_scheme()` 调用封装与 `soundScheme` 设置字段镜像 — 验证：前端构建/lint 通过 <!-- comet-task:5ded6c17-7955-49dc-835b-1291db082d2b -->
- [x] 4.2 `Audio.svelte` 提示音组新增「音效方案」单选行（简洁/自定义），变更经 `save({ soundScheme })` 即时持久化 — 验证：切换后 `settings.json` 中 `soundScheme` 更新，重启应用后保持 <!-- comet-task:af744057-e00b-4c16-913d-bd212b744ced -->
- [x] 4.3 简洁方案下渲染只读固定映射行（事件名 + 音效名 + 试听按钮）并隐藏逐事件选择；切回自定义恢复逐事件行且保留原有选择 — 验证：界面切换核对两种形态；试听按钮播放当前方案下该事件实际音效 <!-- comet-task:ca8a65a3-fca5-46dd-b106-710e75105faa -->

## 5. 集成验证

- [x] 5.1 按 `specs/audio-feedback/spec.md` delta 场景逐条端到端验收：方案切换即时生效、简洁固定映射发声、自定义逐事件发声、取消静音、总开关全静音、试听行为、设置持久化、缺省字段兜底 — 验证：逐场景手动验收并记录结果 <!-- comet-task:d703345d-8558-41ad-a2ea-0b1128ff3c41 -->
- [x] 5.2 回归默认路径：使用不含 `soundScheme` 字段的旧 `settings.json` 启动，确认行为与升级前一致（默认自定义方案） — 验证：旧配置启动核对逐事件行为与持久化 <!-- comet-task:625b9c47-7031-418c-b55e-32ac33bf62dd -->

## 6. 纯双音复刻（走查后范围修订，用户已确认）

- [x] 6.1 Rust 侧纯双音与 macOS 系统音：`SIMPLE_SCHEME` 收敛为开始/处理两映射（完成/失败在简洁方案下不发声）；macOS 经 `objc2-app-kit` NSSound 在主线程播系统音 Tink/Pop（不可用时回退内置近似音），Windows 直接用内置近似音；试听按当前方案与平台解析实际音源 — 验证：`cargo test` 新增用例通过（两映射、Done/Error 简洁不发声、rows 仅两行） <!-- comet-task:7ac03056-de76-41eb-a3f0-201c7ec7acf9 -->
- [x] 6.2 前端与授权清单同步：简洁方案映射区仅渲染开始/处理两行并追加「完成与失败不发声」说明行，试听按钮播实际音源；LICENSE.md 的 ding/soft-alert 用途改回「可选」、tink/pop 标注为简洁方案兜底音 — 验证：`npm run build` 通过，界面核对两种形态 <!-- comet-task:911fad23-0035-4e73-9d23-4b17f12eb909 -->
- [x] 6.3 开始音可闻性修复（走查缺陷）：`start_recorder` 时序对齐 cometix-dictation（开麦/转正 → 播开始音 → 延迟压音量），预开麦路径转正时先还原音量再发声；压音量按 `sound::play_duration`（系统音已知时长 / WAV 时长 RIFF 块链解析 + 投递余量）延迟到提示音播完，且仅在会话进行中生效；两个方案的开始音均不再被「录音时调整音量」压掉 — 验证：用户实机走查确认开始音完整可闻；`cargo test` 全量通过（现有测试基建无法对时序做 RED；WAV 时长解析有 RIFF 块链与真实文件回归单测——最终集成审查曾发现固定偏移对真实文件返回 0ms，已修复，如实记录） <!-- comet-task:c52d5248-9af6-425b-9c93-eb8bde365ac1 -->

## Workflow follow-up

- Verify 阶段通过后由 `/comet-archive` 归档，同步 `docs/openspec/specs/audio-feedback/spec.md`
- 归档与交付方式（local/push/pr）在 Archive 阶段按用户选择执行

```

## docs/openspec/changes/simple-sound-scheme/.openspec.yaml

- Source: docs/openspec/changes/simple-sound-scheme/.openspec.yaml
- Lines: 1-2
- SHA256: c6ab3a95d8d0fd469ca8750ef79234025bf4a8356d5d57856e97d7e47fc8b6c2

```md
schema: spec-driven
created: 2026-10-31

```

## docs/openspec/changes/simple-sound-scheme/specs/audio-feedback/spec.md

- Source: docs/openspec/changes/simple-sound-scheme/specs/audio-feedback/spec.md
- Lines: 1-109
- SHA256: 100fd6155f99182b81b85f3942507b50f6da87a5613358931e6ce725e964f5bd

[TRUNCATED]

```md
# Spec Delta

## MODIFIED Requirements

### Requirement: 会话事件提示音

应用必须在录音会话的四个关键事件（录音开始、进入识别处理、处理完成（结果就绪）、识别失败（含转写超时与出错））遵循用户选择的音效方案与总开关发出提示音：简洁方案仅开始与处理发声（纯双音，见下述场景），完成与失败不发声；自定义方案四事件均按逐事件选择发声。取消录音时不得播放任何提示音。

#### Scenario: 录音开始发声

- **WHEN** 用户通过热键或托盘触发一次正式录音开始
- **THEN** 播放「录音开始」事件在当前音效方案下使用的提示音

#### Scenario: 进入识别处理发声

- **WHEN** 录音结束、应用进入识别处理状态
- **THEN** 播放「处理」事件在当前音效方案下使用的提示音

#### Scenario: 处理完成发声

- **WHEN** 识别结果就绪（单候选直接注入，或多候选面板呈现）
- **THEN** 播放「完成」事件在当前音效方案下使用的提示音

#### Scenario: 识别失败发声

- **WHEN** 识别失败、出错或转写超时
- **THEN** 播放「失败」事件在当前音效方案下使用的提示音

#### Scenario: 简洁方案开始与处理按固定映射发声

- **WHEN** 音效方案为简洁，录音开始或进入识别处理
- **THEN** 录音开始播轻巧"叮"声、进入识别处理播轻弹"啵"声；macOS 上播放系统提示音 Tink 与 Pop（与 cometix-dictation 一致），Windows 上播放内置近似音；系统音不可用时回退内置近似音

#### Scenario: 简洁方案完成与失败不发声

- **WHEN** 音效方案为简洁，识别完成或识别失败
- **THEN** 不播放提示音（纯双音极简风格）

#### Scenario: 自定义方案逐事件发声

- **WHEN** 音效方案为自定义
- **THEN** 各事件播放用户为该事件选中的音效，四个事件的发声互不干扰

#### Scenario: 取消录音不发声

- **WHEN** 用户在识别完成前取消录音会话
- **THEN** 不播放任何提示音

### Requirement: 提示音设置

音频设置页必须提供提示音设置组：音效方案选择控件（简洁、自定义两选一，默认自定义），以及提示音总开关（默认开启）。自定义方案下必须提供录音开始、处理、完成、失败四个事件各自独立的音效选择控件，音效必须从内置音效集合中选择；简洁方案下必须以只读方式展示发声事件（录音开始、进入识别处理）的固定音效映射，并明确标注完成与失败不发声，且不得提供逐事件音效选择控件。音效方案、总开关与逐事件选择的任一设置变更必须即时生效并持久化。

#### Scenario: 关闭总开关后全静音

- **WHEN** 提示音总开关处于关闭状态
- **THEN** 无论音效方案为何，四个事件均不播放提示音，其余功能行为不变

#### Scenario: 方案切换即时生效

- **WHEN** 用户在简洁与自定义之间切换音效方案
- **THEN** 后续会话事件立即按新方案的规则发声，无需重启应用

#### Scenario: 简洁方案隐藏逐事件选择

- **WHEN** 音效方案为简洁
- **THEN** 设置页只读展示开始与处理的固定音效映射，标注完成与失败不发声，不展示逐事件音效选择控件

#### Scenario: 自定义方案恢复逐事件选择

- **WHEN** 音效方案切回自定义
- **THEN** 四个事件的音效选择控件恢复显示，且各事件保持此前已保存的选择

#### Scenario: 逐事件选择独立生效

- **WHEN** 音效方案为自定义，用户为某一个事件选择不同的音效
- **THEN** 该事件改播新选音效，其余事件保持各自原有音效

#### Scenario: 设置持久化

- **WHEN** 用户修改音效方案、总开关或逐事件音效选择后重启应用

```

Full source: docs/openspec/changes/simple-sound-scheme/specs/audio-feedback/spec.md
