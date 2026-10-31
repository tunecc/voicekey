---
change: audio-feedback-sounds
design-doc: docs/openspec/changes/audio-feedback-sounds/design.md
base-ref: 59d715b310b8a7f9e1e9751b7e56f229be40502b
archived-with: 2026-10-31-audio-feedback-sounds
---

<!-- comet-task-authority: docs/openspec/changes/audio-feedback-sounds/tasks.md -->

# 实施计划：音频反馈提示音

依据：design.md（D1–D7）与 specs/audio-feedback/spec.md。执行策略 autonomous + tdd + standard。
所有音效 ID 以 `sound.rs` 常量表为单一来源；任务 2.2 的字段名/默认值与任务 4.2 的 TS 字段保持镜像。

## 1. 音效资源与打包

<!-- comet-task-ref:68d16b86-eb0a-4d5f-9fd4-9dca190621f7 -->
- **1.1 引入 CC0 音效素材**：下载 Kenney Interface Sounds（kenney.nl，CC0 1.0）等授权干净的素材包，挑选 6 个短促候选（目标语义：pop / blip / chime / alert / ding / swoosh），用 `afconvert` 转 44.1kHz/16-bit/mono WAV 放入 `app/src-tauri/resources/sounds/<id>.wav`；新增 `app/src-tauri/resources/sounds/LICENSE.md` 逐文件记录来源与授权。验收：清单逐文件列出来源；`git status` 仅预期新增。依赖：无。

<!-- comet-task-ref:b2b34c82-1ce5-4120-ad5c-d5781b0bf8e4 -->
- **1.2 资源打包与定位**：`tauri.conf.json` 增加 `"resources": ["resources/sounds/*"]`；`sound.rs` 提供 `fn sound_path(app, id) -> Option<PathBuf>`（`BaseDirectory::Resource` 解析 `resources/sounds/<id>.wav`，缺失返回 None 由调用方静默跳过）。验收：`cargo check` 通过；dev 运行日志可解析到文件。依赖：1.1。

## 2. 播放引擎与设置字段

<!-- comet-task-ref:9a50e35e-8b87-4188-a69a-cdc54d748cba -->
- **2.1 sound.rs 播放引擎**（TDD）：`app/src-tauri/Cargo.toml` 加 `rodio`（`default-features = false` + WAV 解码特性）；新增 `src/sound.rs`：
  - `pub const SOUNDS: &[(&str, &str)]`（id, 中文名）；`pub enum SoundEvent { Start, Processing, Done, Error }`
  - `pub fn play(app: &AppHandle, event: SoundEvent)`：读 `AppState.settings`，`sound_enabled == false` 直接返回；按事件取 `settings.sound_*` 播放
  - `pub fn play_id(app: &AppHandle, id: &str)`：试听路径，不做开关判断；未知 ID 静默返回
  - 播放实现：`OutputStream` 懒初始化存 `OnceLock`；`Sink::try_new` → `Decoder::new_wav(BufReader<File>)` → `sink.append`；Sink 存全局 `Mutex<Vec<Sink>>` 并在每次播放前 `retain(!s.empty())`；所有错误 `eprintln!` 后返回，不 panic、不向上传播
  - RED：先写 `SOUNDS` 完整性测试（ID 非空唯一、`resources/sounds/<id>.wav` 存在）与事件→ID 映射/开关门控的纯逻辑测试（将门控逻辑抽为可测函数 `fn event_id<'a>(event, s: &'a Settings) -> Option<&'a str>`），确认因待实现而失败；GREEN：实现后通过。验收：`cargo test -p voicekey` 全绿。
  依赖：1.2。

<!-- comet-task-ref:267b79dd-fa37-4fff-8917-27b13362cecf -->
- **2.2 设置字段**（TDD）：`settings.rs` `Settings` 增 `sound_enabled: bool`（默认 true）与 `sound_start/sound_processing/sound_done/sound_error: String`（默认取 D2 映射 ID）；`sanitize()` 对四个 ID 调 `sound::is_known(id)`，未知回退该事件默认；`api.ts` `Settings` interface 增 `soundEnabled: boolean; soundStart: string; soundProcessing: string; soundDone: string; soundError: string`。RED：旧配置 JSON（无新字段）反序列化默认值 + 未知 ID sanitize 回退，先失败后通过。验收：`cargo test -p voicekey` 全绿；`npm run build`（前端 tsc）通过。依赖：2.1（is_known/常量表）。

## 3. 会话事件接线

<!-- comet-task-ref:db84e1f1-f649-4a70-9cc2-254a03013a35 -->
- **3.1 触发点接线**（按 design.md D4 映射表逐点接入，人工验证）：
  - Start：`start_recorder()` 成功汇合处一次（覆盖 begin/resume/begin_multi 与 take_primed）；开麦失败分支发 Error
  - Processing：`end()` 在 `stop_recording()` 后、非 Idle 分支一次
  - Done：单候选 `done()` 定稿成功且有文本（流式与非流式）；Pick/Compare `finished` 且存在 Final 行时一次
  - Error：`abort(message.is_some())`（转写超时）、单候选定稿出错或空文本、Pick `finished` 且无 Final 行
  - 不发声：`abort(None)`/`cancel()`/`disarm()`、`arm()` 失败、行级错误
  验收：dev 手动走查上述场景（含断网制造失败）；`cargo check` 通过。依赖：2.1、2.2。

<!-- comet-task-ref:cd0b6b8d-f70b-4280-a4a2-196a308049ca -->
- **3.2 行为一致性确认**：关闭总开关重复 3.1 场景，确认与现版本行为一致（不发声、无其他变化）；`cargo test` 全量通过。验收：走查记录 + 测试全绿。依赖：3.1。

## 4. 设置界面与试听

<!-- comet-task-ref:d33aa40a-534b-4eb8-98ff-25915a775ee3 -->
- **4.1 preview_sound / list_sounds 命令**：`main.rs` 增两个 `#[tauri::command]` 并注册；`api.ts` 增 `previewSound(id)` 封装。验收：dev 下命令调用出声；`cargo check` 通过。依赖：2.1。

<!-- comet-task-ref:34bed515-10d1-45da-9e0e-a837666e267f -->
- **4.2 Audio.svelte 提示音组**：新增第二个 `.group`：总开关行（`save({soundEnabled})`）+ 四事件行（`select.sel` 选项来自 `list_sounds()` + `.btn` 试听按钮播当前选中项）；沿用现有样式类，页面内样式仅新增必要项。验收：界面操作——切换开关、换音效、试听即时出声、重启保留。依赖：2.2、4.1。

## 5. 集成验证

<!-- comet-task-ref:5481c12f-150d-4f88-99ee-8e5a126fd379 -->
- **5.1 规格走查**：按 specs/audio-feedback/spec.md 13 个场景逐项验证并记录结果（含缺省字段兜底：手工删除 settings.json 新字段后启动；播放失败静默：临时移走一个 wav 文件）。验收：走查清单逐项通过。依赖：4.2。

<!-- comet-task-ref:cfda67bc-b8a9-4625-b05d-d3a86e292157 -->
- **5.2 桌面构建**：`cd app && npm run tauri build`（macOS）；确认产物 bundle 含 `resources/sounds/*.wav` 且安装后可播放。Windows 一致性交由 CI。验收：构建成功、产物含音效。依赖：5.1。

## 验证与审查说明

- 构建证据录制：`comet check run audio-feedback-sounds build --local --cwd app -- npm run build`（前端+tsc 快速反馈）与 `cargo test`（`--cwd app/src-tauri`）；guard 前以完整命令复跑。
- standard 审查：实施完成后由独立审查者审查全部 diff（重点：controller.rs 触发点映射与 D4 一致性、播放失败静默、设置字段 sanitize），问题按 CRITICAL/IMPORTANT/其他分级处理。
