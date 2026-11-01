# Tasks

## 1. 合并上游 main

- [x] 1.1 执行 `git fetch upstream && git merge upstream/main`，冲突逐个按 design.md 决策表处理，产出 merge commit。验证：`git status` 干净且 merge commit 在 main 历史中 <!-- comet-task:7a133d0a-07f7-46d5-b7fc-bbf0c7a31419 -->
- [x] 1.2 对照 9 个交集文件（design.md 决策表）逐文件核对取舍：音量相关采用上游，fork 特有功能（录音提示音、菜单栏图标、自有更新源）代码仍在。验证：逐文件 `git diff` 审查记录 + grep 确认 fork 特性代码存在 <!-- comet-task:3c56499b-f56c-4c08-b91c-9486101ca74e -->

## 2. 切换音量实现并清理旧实现

- [x] 2.1 删除 `crates/core/src/audio/volume.rs` 与 `crates/core/src/audio.rs` 中的 `pub mod volume;`，移除 `controller.rs` 中 `lower_volume()`/`restore_volume()` 及全部调用点。验证：`cargo check -p voicekey-core` 通过，仓库内 `grep -r RecordingVolume` 无结果 <!-- comet-task:d9cafde0-336f-4abc-b801-92c72b1e0ffb -->
- [x] 2.2 `app/src-tauri/src/settings.rs` 删除 `volume` 字段、采纳上游 `lower_system_volume`/`lower_system_volume_percent`，`app/src/lib/api.ts` 对齐上游字段。验证：`cargo check --workspace` 通过 <!-- comet-task:da347451-03e0-4f19-ab38-4e2876bdafdf -->
- [x] 2.3 `app/src/pages/Audio.svelte` 音量行替换为上游「录音时降低音量」UI（系统音频分组）。验证：`npm run build` 通过 <!-- comet-task:27d6167d-be27-4cf6-ad32-c00d6ed05eee -->
- [x] 2.4 `README.md` 音量功能描述更新为上游行为（按比例降低、默认关闭）。验证：README 中不再有固定值 15% 描述 <!-- comet-task:93bd4e73-520b-4f14-8efc-52a9701854c6 -->

## 3. 麦克风修复去重

- [x] 3.1 `crates/core/src/audio.rs` 与 `crates/core/src/audio/devices.rs` 以上游 #7（`6011a19`）文本为准合并，删除 fork #1 的重复实现，保留 fork 的 `devices::running()` 帮助函数与 live 测试。验证：`cargo test -p voicekey-core` 通过 <!-- comet-task:28a1c26a-bc26-488b-b132-be6eae577348 -->

## 4. 验证

- [x] 4.1 `cargo test --workspace` 与 `cargo clippy --workspace` 全部通过。验证：命令退出码 0 <!-- comet-task:e9fc7136-34ed-483d-837b-b311dac42835 -->
- [x] 4.2 `npm ci && npm run build` 通过。验证：前端生产构建成功 <!-- comet-task:07375266-9a0f-4b9e-8041-d05055fda909 -->
- [x] 4.3 实机音量验证：开启功能（比例 70%），系统音量 50% → 录音中约 15% → 结束还原 50%；短按取消音量不变；录音中手动调节后结束保留用户值。验证：本机 CoreAudio 实测记录 <!-- comet-task:df0c8821-00ef-4567-ab6b-d38a52acf14d -->
- [x] 4.4 旧配置兼容冒烟：settings.json 含 `volume: 15` 时应用正常启动、功能默认关闭。验证：本机实测记录 <!-- comet-task:dc6765d0-589e-4814-958f-5d3504525b0b -->
- [x] 4.5 fork 特性冒烟：录音提示音、菜单栏图标、更新检查各验证一次正常。验证：本机实测记录 <!-- comet-task:4b726857-c250-452d-a466-aa5dbc8963da -->

## Workflow follow-up

- 归档与交付（提交、push 到 origin main）按 `/comet-archive` 的确认流程执行，不在本任务清单内跟踪
