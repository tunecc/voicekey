# Design

## 目标与非目标

- 目标：把提示音功能以关键文件最小修改集移植到上游最新基线（upstream/main v1.9.0），并在该基线上验证构建成功
- 非目标：不修改本仓库（fork）代码；不携带 fork 其他增强；不迁移更新通道；不改上游版本号与 updater 配置

## PR 分支拓扑

- 基线：`upstream/main`（25b3f50，v1.9.0）；fork 领先 54 提交、落后 3 提交，不能合并/ Cherry-pick 整段历史，必须文件级重放提示音 hunks
- PR 分支：`feat/session-sound-feedback`，位于项目内独立 git worktree `.worktrees/pr-sound-feedback`，经 `.git/info/exclude` 排除（不污染主工作区 status，也不进入 PR）
- 主工作区留在 comet change 分支 `comet/upstream-pr-sound-feedback`，仅承载流程产物；cargo/npm 构建与测试都在 PR worktree 内执行并记录证据（check-policy 声明该 cwd）

## 关键文件清单（PR 全部内容）

新增（自 fork 移植）：

| 文件 | 说明 |
| ---- | ---- |
| `app/src-tauri/src/sound.rs` | 提示音模块：SYSTEM_SOUNDS 集合、逐事件门控、NSSound 主线程播放与按名缓存（重播先 stop 后 play）、rodio WAV 兜底、`play_duration`、`play_id` 试听。移植时删除 `SCHEME_SIMPLE`，注释去掉 fork 内部文档引用 |
| `app/src-tauri/resources/sounds/*.wav` + `LICENSE.md` | 11 个 CC0 WAV：Windows 兜底与 macOS 系统音不可用时兜底 |

修改（上游版本 + 仅提示音 hunks）：

| 文件 | 提示音 hunks |
| ---- | ---- |
| `app/src-tauri/Cargo.toml` | +`rodio`(wav)；macos target +`objc2-app-kit`(NSSound)/`objc2-foundation`(NSString) |
| `Cargo.lock` | 随构建更新 rodio/objc2 等条目并纳入提交 |
| `app/src-tauri/tauri.conf.json` | 仅 `bundle.resources` 增加 `"resources/sounds/*"` |
| `app/src-tauri/src/main.rs` | `mod sound;`；`preview_sound`/`list_sounds` 命令与 invoke_handler 注册（保留上游 `jev_test`） |
| `app/src-tauri/src/settings.rs` | `NONE` 保留字；`sound_enabled`/`sound_start`/`sound_processing`/`sound_done`/`sound_error` 字段与默认值；sanitize 逐事件校验回退循环；`sound_settings_tests` 单测（不含 scheme 两条；样例 JSON 按上游字段调整） |
| `app/src-tauri/src/controller.rs` | `SoundEvent` 导入；`stopped_at`/`chime_gen` 字段；开麦失败发 Error；开麦成功发 Start 并递增代数；开始音播完后经 `Msg::VolumeDucking` 延迟压低；`play_after_restore`/`restore_wait`（450ms）；进入识别处理发 Processing；识别失败/空结果/超时取消(message 存在)发 Error |
| `app/src/lib/api.ts` | `Settings` 接口 +5 个 sound 字段（无 soundScheme）；`previewSound`/`listSounds` |
| `app/src/pages/Audio.svelte` | 提示音设置组：总开关 + 四行逐事件选择器 + 试听按钮（「不发声」行试听置灰） |

## 上游适配点

1. ducking API：上游已合并 PR #8（`OutputVolumeDucker`），`Msg::VolumeDucking { enabled, reduction_percent }` handler 自带 `enabled && self.recording` 门控。移植沿用该消息机制把压低延迟到开始音播完（`play_duration(Start)`），不改 `crates/` 任何代码
2. 恢复窗口：fork 的 450ms 常数与上游 ducker `FADE_MAX_MS`（450ms）一致，`restore_wait` 原样保留，注释说明取自 ducker 淡入上界
3. 排除 fork 的 600ms 电平表重启延迟 hunk（属音量/设备枚举增强，非提示音）
4. 空 if 分支整理：多渠道全失败判定写成 `if !self.rows.iter().any(|r| r.state == RowState::Final) { … }` 单分支，行为不变
5. `sound_scheme` 不移植（字段、默认值、sanitize 强制行、两条 scheme 单测、api.ts 字段都不进 PR）；fork 侧该字段仅供自身旧配置兼容，与本 PR 无关
6. 「识别完成」行如实镜像 fork 现行 spec：默认不发声、结果就绪不另接线（完成信号由处理音承担），不为其新增触发点

## 明确排除（防泄漏清单）

`ui.rs`、`hotkey.rs`、`Keys/Onboard/Settings/Pick/Recog/Jev.svelte`、`store.test.mjs`、`icons/tray*.png`、`tauri.conf.json` 的 updater/pubkey/version、`crates/` 全部、`docs/`。

## 验证

- PR worktree 内：`cargo fmt --check`、`cargo check`、`cargo test --workspace`（含移植的 sound_settings 单测与上游原有测试）
- PR worktree 内：`npm run build`（vite 构建前端）
- 泄漏审查：`git diff upstream/main..HEAD --stat` 逐文件核对 = 关键文件清单（10 项），排除清单零混入
