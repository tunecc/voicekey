# Tasks

## 1. PR 基座

- [x] 1.1 `git fetch upstream`，从 `upstream/main` 创建分支 `feat/session-sound-feedback` 于独立 worktree `.worktrees/pr-sound-feedback`，并把该路径写入 `.git/info/exclude`。验证：worktree 内 `git log -1 --oneline` = 25b3f50，主工作区 `git status` 干净（无未跟踪 worktree 目录）
- [x] 1.2 新增 `app/src-tauri/src/sound.rs`（对照 fork 版删除 `SCHEME_SIMPLE` 常量与 fork 内部文档引用注释）与 `app/src-tauri/resources/sounds/`（11 个 WAV + LICENSE.md）。验证：与 fork 版本 diff 仅 scheme 常量与注释差异，WAV 文件逐一相同（`git diff --no-index` 校验或 md5）

## 2. 移植修改（上游版本 + 仅提示音 hunks）

- [x] 2.1 `app/src-tauri/Cargo.toml` 增加 rodio 与 macos target 的 objc2-app-kit/objc2-foundation；`tauri.conf.json` 仅 `bundle.resources` 加 `"resources/sounds/*"`。验证：`git diff` 仅这两处 hunk，version/updater 未动
- [x] 2.2 `main.rs`：`mod sound;`、`preview_sound`/`list_sounds` 命令与 invoke_handler 注册。验证：保留上游 `jev_test`，新增项与 fork 对应 hunk 一致
- [x] 2.3 `settings.rs`：`NONE` 常量、五个 sound 字段与默认值、sanitize 逐事件校验循环、`sound_settings_tests`（去掉 scheme 两条，样例 JSON 按上游字段）。验证：worktree 内 `cargo test -p voicekey sound_settings` 通过
- [x] 2.4 `controller.rs`：`SoundEvent` 导入、`stopped_at`/`chime_gen` 字段、开麦失败 Error、开麦成功 Start + 代数递增、开始音播完后经 `Msg::VolumeDucking` 延迟压低（不改上游 handler 与 crates/）、`play_after_restore`/`restore_wait`(450ms)、进入识别处理 Processing、失败路径 Error、全失败判定取反单分支。验证：与 design 适配点 1-4 逐条核对一致
- [x] 2.5 `api.ts`：Settings 接口 +5 个 sound 字段（无 soundScheme）、`previewSound`/`listSounds`；`Audio.svelte`：提示音设置组（总开关 + 四行选择器 + 试听、「不发声」行试听置灰）。验证：无 fork 其他前端增强混入

## 3. 构建与测试（上游基线）

- [x] 3.1 worktree 内 `cargo fmt --check && cargo check && cargo test --workspace` 通过；`Cargo.lock` 随之更新并纳入提交。验证：退出码 0
- [x] 3.2 worktree 内 `npm install && npm run build`（vite）通过。验证：退出码 0
- [x] 3.3 泄漏审查：`git diff upstream/main..HEAD --stat` 逐文件核对 = design 关键文件清单 10 项，排除清单零混入。验证：核对记录写入验证报告

## 4. 交付物

- [x] 4.1 PR 分支提交（单个 `feat:` 提交，仅含关键文件），PR 标题与描述草稿写入本 change 目录 `pr-draft.md`。验证：`git show --stat` 仅含关键文件
