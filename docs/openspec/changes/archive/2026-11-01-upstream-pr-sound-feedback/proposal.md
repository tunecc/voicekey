# Proposal

## Why

fork 的「会话提示音」功能（录音各阶段系统铃声反馈 + 设置页逐事件选择与试听）已在本仓库稳定使用，上游 J3n5en/voicekey 没有该功能。需要把该功能以最小、干净的修改集 PR 到上游：只包含提示音功能本身的关键文件改动，不携带 fork 后的其他增强（长按「无」、托盘触发文案、电平表延迟重启、自有音效方案兼容层等），并必须在上游最新代码基线上验证构建成功。

## What Changes

- 以上游 main（v1.9.0，25b3f50）为基线创建 PR 分支（独立 git worktree，不触碰主工作区），把提示音功能重新应用其上：
  - 新增 `app/src-tauri/src/sound.rs`（提示音模块：macOS NSSound 系统铃声、非 macOS / 系统音缺失时 CC0 WAV 兜底、逐事件门控、试听播放）与 `app/src-tauri/resources/sounds/`（11 个 CC0 WAV + LICENSE）
  - 修改 `Cargo.toml`（rodio + objc2-app-kit/objc2-foundation）、`Cargo.lock`、`tauri.conf.json`（仅 bundle.resources）、`main.rs`（模块声明与 preview/list 命令）、`settings.rs`（sound_enabled + 四个逐事件字段、默认值、sanitize 与单测）、`controller.rs`（开始/处理/失败触发点、会话代数门闩、开始音播完再压低音量、恢复窗口后播处理/失败音）、`app/src/lib/api.ts`、`app/src/pages/Audio.svelte`（提示音设置面板 + 逐行试听）
- 默认可听行为与 fork 现状完全一致：录音开始=Tink、识别处理=Pop、识别完成/失败不发声；「识别完成」行按 fork 现状保留（完成信号由处理音承担，结果就绪不另接线发声，符合现行 spec）
- 为上游基线做的整理：不移植 fork 自有的 `sound_scheme` 字段（上游从未发布过旧方案，无兼容包袱）；controller 空 if 分支整理为取反单分支；注释中指向 fork 内部文档的引用改为自包含描述
- 在 PR worktree 内以上游基线完成 cargo check/test 与前端 vite 构建验证，并逐文件审查确认无其他 fork 改动泄漏

## Capabilities

### New Capabilities

（无——本 change 不改变本仓库行为，交付物是上游 PR 分支）

### Modified Capabilities

（无——fork 自身代码零改动）

## Impact

- 本仓库：仅 `docs/openspec` 流程产物；代码零改动
- 交付物：基于 upstream/main 的 PR 分支 `feat/session-sound-feedback`（位于独立 worktree）+ 分支提交与 PR 标题/描述草稿
- 明确排除（防泄漏）：`ui.rs`、`hotkey.rs`、`Keys/Onboard/Settings/Pick/Recog/Jev.svelte`、`store.test.mjs`、tray 图标、`tauri.conf.json` 的 updater endpoints/pubkey 与 version、`crates/` 全部改动、600ms 电平表重启延迟
