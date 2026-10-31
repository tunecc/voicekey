---
comet_change: audio-feedback-sounds
role: technical-design
canonical_spec: openspec
archived-with: 2026-10-31-audio-feedback-sounds
status: final
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

- [rodio 新增依赖带来编译体积与 Windows 后端风险] → 底层 cpal 已在依赖树中（音频捕获使用），rodio 复用同一后端；`default-features = false` 精简；CI 桌面构建（macOS universal + Windows）作为验证关口。
- [音效素材授权瑕疵] → 仅引入 CC0/公有领域素材，随包附带来源与授权清单；构建任务中核对每个文件的授权声明。
- [开始音被录入识别缓冲] → 短促音色 + 既有音量压低机制缓解；记录为已知取舍（D6）。
- [提示音与识别结果注入抢音频焦点] → rodio 走系统默认输出混音，不独占设备；提示音时长短，与音频捕获（输入设备）互不冲突。
- [旧配置中残留未知音效 ID] → `sanitize()` 回退默认值，不报错。
- [Sink 注册表积累] → 每次播放时清理已结束项。

## Migration Plan

纯增量变更：`settings.json` 新增字段由 serde 默认值兜底，无需迁移；回滚等同关闭总开关或还原代码。不涉及数据库、网络协议或公共 API。

## 测试策略

- Rust 单元测试（随各任务落地）：音效 ID 常量表完整性（ID 唯一、每个 ID 有对应资源文件）；`sanitize()` 未知 ID 回退默认值；旧配置 JSON（无新字段）反序列化后得到默认值。
- 手动走查（dev 模式）：按 specs/audio-feedback/spec.md 场景清单逐项听音验证；含关闭总开关后与现版本行为一致性的对比走查。
- 构建：本地 `npm run tauri build`（macOS）确认音效资源进入产物；Windows 表现一致性由 CI 桌面构建覆盖。

## Open Questions

- 具体音色挑选与默认映射（构建阶段从 CC0 素材库试听后确定，不改变结构与任务拆分）。
