# Comet Design Handoff

- Change: audio-feedback-sounds
- Phase: design
- Mode: compact
- Context hash: a94ffa047692b9aefbe7c3fb7d5f5698b67567b245c80b51920a74e4138cd901

Generated-by: comet-handoff.sh
Task hash policy: task-content-v1. Read tasks.md for live completion; excerpts are design-time context.

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## docs/openspec/changes/audio-feedback-sounds/proposal.md

- Source: docs/openspec/changes/audio-feedback-sounds/proposal.md
- Lines: 1-39
- SHA256: b4902507a37f2879b293384db11214efe58c08bf850de423600e8af4b2e39f46

```md
# Proposal

## Why

语音输入期间用户的视线经常不在屏幕上：按下热键后，录音是否开始、是否已进入识别、结果是否就绪，目前只能依赖 HUD 浮层和托盘图标的视觉变化来确认；识别失败时用户尤其容易盯着别处干等。提示音是同类语音输入工具（如 cometix-dictation 的开始/停止音）验证过的低成本反馈手段，可以在不增加视觉负担的情况下让用户"听到"当前阶段。

## What Changes

- 新增音频反馈提示音能力：在录音会话的四个关键事件播放提示音——录音开始、进入识别处理、处理完成（结果就绪）、识别失败/超时。
- 音频设置页（`音频` 分区）新增「提示音」设置组：
  - 一个总开关，默认开启；
  - 每个事件一个独立的音效选择控件，可从内置音效中任选；
  - 每个音效选择控件旁带一个试听按钮，点击立即播放所选音效。
- 内置一组开源授权（CC0/MIT 等允许再分发的授权）音效文件随应用打包，macOS 与 Windows 表现一致；不依赖系统音效目录。
- 提示音设置持久化到现有 `settings.json`；旧配置文件缺少新字段时按默认值兜底，无迁移成本。
- 提示音播放失败时静默降级，绝不影响录音与转写主流程；取消录音不发声。
- 不做独立音量调节（提示音跟随系统输出音量）；不做自定义音效导入；不改动遗留 Swift 菜单栏应用与 iOS/Android 客户端。

## Capabilities

### New Capabilities

- `audio-feedback`: 录音会话各阶段的音频反馈——哪些事件发声、设置（总开关与逐事件音效选择）、试听行为、持久化与播放失败降级。

### Modified Capabilities

（无——本项目尚无既有能力规格。）

## Impact

- **代码**
  - `app/src-tauri/src/settings.rs`：`Settings` 结构新增提示音相关字段（总开关、逐事件音效选择）+ 默认值 + TS 侧 `app/src/lib/api.ts` 类型镜像。
  - `app/src-tauri/src/controller.rs`：在录音开始（`start_recorder`/`take_primed`）、进入识别（`end()` 的 wait 转换）、完成/失败（`done()` 各分支）处接入提示音触发。
  - 音效播放：新增播放实现（Rust 侧播放或前端 webview 播放，具体方案在 design 阶段定夺）。
  - `app/src/pages/Audio.svelte`：音频设置页新增「提示音」设置组（开关、逐事件下拉、试听按钮）。
- **资源与打包**：新增内置音效文件及打包配置（前端静态资源或 Tauri `bundle.resources`，随播放方案确定）。
- **依赖**：若选择 Rust 侧播放，需新增音频播放依赖（如 `rodio`）；前端播放则无新增依赖。
- **平台**：仅 Tauri 桌面应用（macOS + Windows）；不涉及 `Sources/VoiceKey`（遗留 Swift 应用）、`ios/`、`android/`。
- **兼容性**：`settings.json` 仅新增字段，serde 默认值兜底，无破坏性变更；无公共 API 变更。

```

## docs/openspec/changes/audio-feedback-sounds/design.md

- Source: docs/openspec/changes/audio-feedback-sounds/design.md
- Lines: 1-100
- SHA256: 9588706f4e704ed6c78f66a501dfa0fd352a85e3a4e7545530ea89a0d07d4ca4

[TRUNCATED]

```md
---
comet_change: audio-feedback-sounds
role: technical-design
canonical_spec: openspec
---

# Design

## Context

voicekey 桌面应用（`app/`，Tauri 2：Rust 后端 + Svelte 5 前端）目前没有任何提示音机制：录音会话状态（listen → wait → final/error）只通过 HUD 浮层与托盘图标做视觉反馈。录音生命周期集中在 `app/src-tauri/src/controller.rs` 的状态机中；设置持久化为 Rust 侧 `Settings` 结构（`settings.rs`）写入 `settings.json`，前端通过 `settings` 事件回写；音频设置页是 `app/src/pages/Audio.svelte`。音频采集走 `crates/core`（cpal 0.15）。目标平台 macOS + Windows。动机见 proposal.md，行为要求见 specs/audio-feedback/spec.md；本设计经用户确认（2026-10-31，摘要见 `.comet/handoff/brainstorm-summary.md`）。

## Goals / Non-Goals

**Goals:**

- 在 Rust 状态机的会话事件点可靠触发提示音，不依赖任何前端窗口的存活状态
- 提供一套随应用打包、授权可再分发的内置音效，macOS/Windows 表现一致
- 设置项（总开关 + 逐事件音效）沿用现有 `Settings` 模式，前端设置页可配置、可试听

**Non-Goals:**

- 不做独立音量调节、自定义音效导入、按会话类型区分音效
- 不改动遗留 Swift 应用与 iOS/Android 客户端
- 不为提示音引入后台常驻音频服务或复杂混音逻辑

## Decisions

### D1：Rust 侧播放（rodio），不走前端 webview

提示音由 Rust 状态机直接触发：新增 `app/src-tauri/src/sound.rs`，基于 `rodio` 播放（rodio 底层即项目已在用的 cpal 音频栈，macOS CoreAudio / Windows WASAPI）。理由：事件源在 controller，且 HUD/设置窗口都可能隐藏或未打开——隐藏 webview 的 JS 执行存在节流与 App Nap 风险，前端播放可靠性不足；Rust 播放与窗口状态无关。设置页试听通过新增 Tauri 命令 `preview_sound(id)` 复用同一播放引擎，保证"试听即所得"，且不受总开关约束（试听是显式用户动作）。

- 备选（已否决）：前端 `new Audio()` 播放——零新增依赖，但触发需 Rust→前端事件转发，且隐藏窗口下行为不可靠。
- 备选（已否决）：系统音效（cometix-dictation 的 `NSSound(named:)` 方案）——仅 macOS 可用，Windows 无等价 API，无法满足跨平台一致的规格要求。

依赖引入：`rodio` 以 `default-features = false` + 仅启用 WAV 解码特性，控制编译体积（底层 cpal 已在依赖树）。

### D2：音效以 WAV (PCM 16-bit) 随 Tauri bundle 打包

格式选 WAV：rodio 内置解码、无额外 decoder 依赖；提示音均短促（0.2–0.5s），单文件几十 KB。存放于 `app/src-tauri/resources/sounds/<id>.wav`，在 `tauri.conf.json` 的 `bundle.resources` 登记该目录（当前未配置该段），运行时经 Tauri `BaseDirectory::Resource` 定位。

音效池约 6 个候选，组成共用池，四个事件各自任选其一；素材从 CC0/公有领域素材库（如 Kenney Interface Sounds）选取，用系统自带 `afconvert` 转为 44.1kHz/16-bit/mono WAV，包内附授权与来源清单。音效 ID 与中文名以 `sound.rs` 中的常量表为单一来源，具体音色在构建阶段试听后确定（默认映射：开始=pop 类、处理=blip 类、完成=chime 类、失败=alert 类）。

### D3：设置字段为扁平 camelCase，沿用 `volume` 字段三件套惯例

`Settings` 新增 5 个字段（`settings.rs` + `app/src/lib/api.ts` 镜像）：

- `sound_enabled: bool`（默认 true）——提示音总开关
- `sound_start` / `sound_processing` / `sound_done` / `sound_error: String`——各事件音效 ID，默认分别为 pop/blip/chime/alert 类

`Default` 提供默认值；`sanitize()` 对四个 ID 字段校验：不在音效常量表中则回退该事件默认值。旧配置缺字段由 `#[serde(default)]` 兜底。备选（已否决）：嵌套 `sound` 对象——与现有全部扁平字段的风格不一致。

### D4：触发点与会话事件映射（经代码核实）

播放封装为 `sound::play(&AppHandle, SoundEvent)`：内部读取实时设置（总开关关闭即跳过），fire-and-forget，失败仅 debug 日志。

| 事件 | 触发点 | 规则 |
| --- | --- | --- |
| Start | `start_recorder()` 成功路径（两处汇合点） | 覆盖 `begin`/`resume`/`begin_multi` 与预开麦 `take_primed` 转正；`resume` 继续说新分段视同一次新的录音开始，同样发声 |
| Start 失败 | `start_recorder()` 的开麦失败分支 | 发失败音（会话已向用户呈现错误）；`arm()` 预开麦失败不发声（会话尚未开始，现状无任何用户可见反馈） |
| Processing | `end()` 在 `stop_recording()` 后、进入 wait 时 | 单候选与 Pick/Compare 统一触发一次；`end()` 对未在录音的状态提前返回，天然去重 |
| Done | 单候选：`done()` 定稿成功且有文本（含流式） | Pick/Compare：全部行出结果（`finished`）时触发一次；面板继续等待用户挑选，挑选上屏不再发声 |
| Error | 转写超时（`abort(Some(..))`）、单候选定稿出错或无文本、Pick 全部行失败、开麦失败 | 空识别结果按失败处理（HUD 本就显示"没有识别到内容"）；Pick/Compare 行级错误不发声（会话仍在继续），全部行失败时发一次 |
| 不发声 | 用户取消（`abort(None)`、`cancel()`、`disarm()`） | 取消保持安静 |

### D5：播放引擎常驻输出流，多 Sink 并行

`OutputStream` 以 `OnceLock` 懒初始化并存活整个应用生命周期（菜单栏应用常驻，成本可忽略）；每次播放新建 `Sink` 追加解码后的音频，存入全局 `Mutex<Vec<Sink>>` 注册表并在每次播放时清理已结束项（防止 Sink 被 drop 导致声音中断，同时不积累）。不做排队/打断——事件间隔远大于单音时长，极端连发时的重叠可接受。音量固定 1.0（跟随系统输出音量）。输出设备在运行中切换不跟随（重启生效）；无可用输出设备时静默跳过。

### D6：开始音与录音捕获重叠的取舍

与 cometix-dictation 相同的已知取舍：开始音触发时麦克风可能已经打开（点按模式的预开麦缓冲中会包含提示音）。采用短促开始音（≤0.3s）把影响降到最低；macOS 上若开启「录音时调整音量」，录制到的提示音也会被同步压低。若后续实测影响识别质量，再评估清空预开麦缓冲或延迟开麦等方案（本 change 不做）。

### D7：命令与前端接线

- `main.rs` 新增两个 Tauri 命令并注册到 `invoke_handler`：`preview_sound(id)`（试听，不受总开关约束，未知 ID 静默忽略）与 `list_sounds()`（返回 `[{id, label}]`，供设置页渲染下拉选项，Rust 常量表为单一来源）。
- `Audio.svelte` 新增「提示音」设置组：总开关行（`.sw`）+ 四个事件行（音效 `select.sel` + 旁边试听 `.btn`），沿用现有 `.group/.row/save()` 惯例；选择变更经 `save()` 即时持久化生效；试听播放该事件当前选中的音效，不修改设置。总开关关闭时各行保持可见（仍可预选，但会话不发声）。

## Risks / Trade-offs


```

Full source: docs/openspec/changes/audio-feedback-sounds/design.md

## docs/openspec/changes/audio-feedback-sounds/tasks.md

- Source: docs/openspec/changes/audio-feedback-sounds/tasks.md
- Lines: 1-26
- SHA256: 1465768ba0c0889f093a6723910f4d35394bd0b94824bf13142c4da9671c6fe1

```md
# Tasks

## 1. 音效资源与打包

- [ ] 1.1 从 CC0/公有领域素材库（如 Kenney UI 音频包）挑选 5–6 个短促候选音效，转为 WAV (PCM 16-bit) 放入 `app/src-tauri/resources/sounds/`，并随包新增授权与来源清单文件。验证：清单逐文件列出来源与授权；`git status` 仅包含预期新增文件。 <!-- comet-task:68d16b86-eb0a-4d5f-9fd4-9dca190621f7 -->
- [ ] 1.2 在 `app/src-tauri/tauri.conf.json` 的 `bundle.resources` 登记音效目录，并在 Rust 侧实现运行时音效文件定位。验证：`npm run dev` 启动后日志可解析出音效文件路径且文件存在。 <!-- comet-task:b2b34c82-1ce5-4120-ad5c-d5781b0bf8e4 -->

## 2. 播放引擎与设置字段

- [ ] 2.1 `app/src-tauri` 引入 rodio 依赖，新增 `src/sound.rs`：`SoundEvent` 枚举、懒初始化常驻输出流（`OnceLock`）、fire-and-forget 播放、播放失败静默（debug 日志、不向上传播）。验证：`cargo check` 通过；模块级单元测试覆盖事件→音效 ID 解析与开关关闭时跳过播放的分支。 <!-- comet-task:9a50e35e-8b87-4188-a69a-cdc54d748cba -->
- [ ] 2.2 `Settings` 新增 `sound_enabled`（默认 true）与 `sound_start`/`sound_processing`/`sound_done`/`sound_error` 字段，补 `Default`；`sanitize()` 将未知音效 ID 回退为该事件默认值；同步 `app/src/lib/api.ts` 的 `Settings` interface。验证：单元测试用无新字段的旧配置 JSON 反序列化后得到默认值；未知 ID 经 sanitize 后回退。 <!-- comet-task:267b79dd-fa37-4fff-8917-27b13362cecf -->

## 3. 会话事件接线

- [ ] 3.1 在 `controller.rs` 接入触发点：录音开始（`start_recorder` 成功与 `take_primed` 转正）、进入处理（`end()` 的 wait 转换，单候选与 Pick/Compare 统一触发一次）、完成（`done()` 成功定稿处，会话级一次）、失败（`done()` 错误/空结果分支与转写超时分支）；`cancel()`/`abort()` 不触发。验证：`npm run dev` 下手动走查——开始/结束/出结果/失败（可临时断网或超时）各发声一次、取消静音。 <!-- comet-task:db84e1f1-f649-4a70-9cc2-254a03013a35 -->
- [ ] 3.2 确认提示音触发不改变会话状态机行为与时序：总开关关闭时各场景表现与现版本一致。验证：关闭开关后重复 3.1 走查；`cargo test` 全量通过。 <!-- comet-task:cd0b6b8d-f70b-4280-a4a2-196a308049ca -->

## 4. 设置界面与试听

- [ ] 4.1 新增 Tauri 命令 `preview_sound(id)` 复用 `sound.rs` 播放引擎（不受总开关约束）。验证：`npm run dev` 下调用命令能听到对应音效。 <!-- comet-task:d33aa40a-534b-4eb8-98ff-25915a775ee3 -->
- [ ] 4.2 `app/src/pages/Audio.svelte` 新增「提示音」设置组：总开关行 + 四个事件行（音效下拉 + 旁边试听按钮），沿用现有 `.group`/`.row`/`save()` 惯例；选择变更即时生效。验证：界面操作——切换开关、逐事件换音效、点击试听即时出声、重启应用后设置保留。 <!-- comet-task:34bed515-10d1-45da-9e0e-a837666e267f -->

## 5. 集成验证

- [ ] 5.1 按规格走查 `specs/audio-feedback/spec.md` 全部场景（含缺省字段兜底、播放失败静默、行为时序不变），记录验收结果。验证：走查清单逐项通过。 <!-- comet-task:5481c12f-150d-4f88-99ee-8e5a126fd379 -->
- [ ] 5.2 本地完成桌面构建（`npm run tauri build`，macOS），确认音效资源进入产物且安装包可正常播放；Windows 一致性交由 CI 桌面构建验证。验证：构建成功、产物含音效资源。 <!-- comet-task:cfda67bc-b8a9-4625-b05d-d3a86e292157 -->

```

## docs/openspec/changes/audio-feedback-sounds/.openspec.yaml

- Source: docs/openspec/changes/audio-feedback-sounds/.openspec.yaml
- Lines: 1-2
- SHA256: c6ab3a95d8d0fd469ca8750ef79234025bf4a8356d5d57856e97d7e47fc8b6c2

```md
schema: spec-driven
created: 2026-10-31

```

## docs/openspec/changes/audio-feedback-sounds/specs/audio-feedback/spec.md

- Source: docs/openspec/changes/audio-feedback-sounds/specs/audio-feedback/spec.md
- Lines: 1-88
- SHA256: 49cbf823e52d0f1ee9fe2ce986fef4a8324ee8f60bea0c72de20da48d766a4ab

[TRUNCATED]

```md
# audio-feedback 变更规格

## ADDED Requirements

### Requirement: 会话事件提示音

应用必须在录音会话的四个关键事件播放提示音：录音开始、进入识别处理、处理完成（结果就绪）、识别失败（含转写超时与出错）。提示音必须遵循用户在设置中选择的总开关与逐事件音效配置；取消录音时不得播放任何提示音。

#### Scenario: 录音开始发声

- **WHEN** 用户通过热键或托盘触发一次正式录音开始
- **THEN** 播放「录音开始」事件当前选中的提示音

#### Scenario: 进入识别处理发声

- **WHEN** 录音结束、应用进入识别处理状态
- **THEN** 播放「处理」事件当前选中的提示音

#### Scenario: 处理完成发声

- **WHEN** 识别结果就绪（单候选直接注入，或多候选面板呈现）
- **THEN** 播放「完成」事件当前选中的提示音

#### Scenario: 识别失败发声

- **WHEN** 识别失败、出错或转写超时
- **THEN** 播放「失败」事件当前选中的提示音

#### Scenario: 取消录音不发声

- **WHEN** 用户在识别完成前取消录音会话
- **THEN** 不播放任何提示音

### Requirement: 提示音设置

音频设置页必须提供提示音设置组：一个总开关（默认开启），以及录音开始、处理、完成、失败四个事件各自独立的音效选择控件。每个事件的音效必须从内置音效集合中选择；任一事件的设置变更必须即时生效并持久化。

#### Scenario: 关闭总开关后全静音

- **WHEN** 提示音总开关处于关闭状态
- **THEN** 四个事件均不播放提示音，其余功能行为不变

#### Scenario: 逐事件选择独立生效

- **WHEN** 用户为某一个事件选择不同的音效
- **THEN** 该事件改播新选音效，其余事件保持各自原有音效

#### Scenario: 设置持久化

- **WHEN** 用户修改提示音设置后重启应用
- **THEN** 总开关状态与各事件的音效选择与重启前一致

#### Scenario: 缺省字段兜底

- **WHEN** 现有配置文件缺少提示音相关字段
- **THEN** 总开关按默认值（开启）生效，各事件使用默认音效，不产生错误

### Requirement: 音效试听

音频设置页中每个事件的音效选择控件旁必须提供试听按钮；点击试听必须立即播放该事件当前选中的音效，且不得修改任何设置。

#### Scenario: 点击试听立即播放

- **WHEN** 用户点击某事件的试听按钮
- **THEN** 立即播放该事件当前选中的音效，且不改变该事件或其他事件的设置

### Requirement: 内置音效与跨平台一致

应用必须内置自带的开源授权（允许再分发）音效集合，不得依赖操作系统音效目录；macOS 与 Windows 上必须提供一致的可选音效集合与播放行为。

#### Scenario: 无系统音效目录依赖

- **WHEN** 在任一受支持平台（macOS 或 Windows）上打开音效选择
- **THEN** 可选音效来自应用内置集合，且两平台的可选集合一致

### Requirement: 主流程不受影响

提示音的播放不得改变录音、识别与结果注入的行为与时序；提示音资源缺失或播放失败时必须静默跳过，不得向用户报错或中断会话。

#### Scenario: 播放失败静默降级

```

Full source: docs/openspec/changes/audio-feedback-sounds/specs/audio-feedback/spec.md
