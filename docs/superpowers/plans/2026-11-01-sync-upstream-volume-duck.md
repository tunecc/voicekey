---
change: sync-upstream-volume-duck
design-doc: docs/openspec/changes/sync-upstream-volume-duck/design.md
base-ref: 6144e16c7e4eb88b84f434db290b869b988e7664
archived-with: 2026-11-01-sync-upstream-volume-duck
---

<!-- comet-task-authority: docs/openspec/changes/sync-upstream-volume-duck/tasks.md -->

# 实施计划：同步上游 + 切换音量方案

配置：autonomous / direct / standard 审查。隔离：current（main 分支）。
需求与验收见 `specs/recording-volume-ducking/spec.md`；合并取舍依据见 `design.md` 决策表。本计划不复制需求，只列执行顺序、命令与验收。

## 1. 合并上游 main

<!-- comet-task-ref:7a133d0a-07f7-46d5-b7fc-bbf0c7a31419 -->
### 1.1 执行合并并解决冲突

- 范围：`git fetch upstream && git merge upstream/main`（预计冲突文件即 design.md 决策表 9 项）
- 约束：不 rebase；冲突解决以决策表为准；merge commit 信息说明同步来源
- 验收：`git status` 干净；merge commit 存在；`cargo check --workspace` 可进入（允许存在待清理错误，由任务 2.x 处理）

<!-- comet-task-ref:3c56499b-f56c-4c08-b91c-9486101ca74e -->
### 1.2 逐文件核对取舍

- 范围：对 9 个交集文件逐一审查 `git diff HEAD^1..HEAD -- <file>` 与 `git diff HEAD^2..HEAD -- <file>`
- 约束：音量相关以上游为准；fork 特性（录音提示音 `chime`/`envelope`、菜单栏图标 `tray`、自有更新源 `updater`、多渠道挑选）代码必须在
- 验收：核对记录写入任务说明；`grep -r` 确认 fork 特性符号存在

## 2. 切换音量实现并清理旧实现

<!-- comet-task-ref:d9cafde0-336f-4abc-b801-92c72b1e0ffb -->
### 2.1 删除旧音量模块与调用点

- 范围：删除 `crates/core/src/audio/volume.rs`；`crates/core/src/audio.rs` 移除 `pub mod volume;`；`controller.rs` 移除 `RecordingVolume`、`lower_volume()`、`restore_volume()` 与全部调用点
- 约束：还原职责交给上游 `OutputVolumeDucker`；结束路径按 design 决策 6 补挂 `set_active(false)`
- 验收：`cargo check -p voicekey-core` 通过；`grep -rn RecordingVolume crates/ app/` 无结果

<!-- comet-task-ref:da347451-03e0-4f19-ab38-4e2876bdafdf -->
### 2.2 设置字段切换

- 范围：`settings.rs` 删除 `volume` 字段，采纳上游 `lower_system_volume`/`lower_system_volume_percent` 与 sanitize/clamp；`app/src/lib/api.ts` 类型对齐
- 验收：`cargo check --workspace` 通过

<!-- comet-task-ref:27d6167d-be27-4cf6-ad32-c00d6ed05eee -->
### 2.3 UI 行替换

- 范围：`Audio.svelte` 音量行采用上游「系统音频」分组写法
- 验收：`npm run build` 通过（在 app/ 目录）

<!-- comet-task-ref:93bd4e73-520b-4f14-8efc-52a9701854c6 -->
### 2.4 README 更新

- 范围：音量功能描述改为「录音时降低音量，按比例、默认关闭」
- 验收：README 无固定值 15% 描述

## 3. 麦克风修复去重

<!-- comet-task-ref:28a1c26a-bc26-488b-b132-be6eae577348 -->
### 3.1 audio.rs / devices.rs 以上游为准

- 范围：保留上游 `6011a19` 的实现文本；fork `860a735` 的 `devices::running()` 与 live 测试若上游没有则保留
- 验收：`cargo test -p voicekey-core` 通过（非 live 测试全绿）

## 4. 验证

<!-- comet-task-ref:e9fc7136-34ed-483d-837b-b311dac42835 -->
### 4.1 workspace 测试与 clippy

- 命令：`cargo test --workspace` && `cargo clippy --workspace`（零新增告警）
- 验收：两命令退出码 0

<!-- comet-task-ref:07375266-9a0f-4b9e-8041-d05055fda909 -->
### 4.2 前端构建

- 命令：`npm ci && npm run build`（app/）
- 验收：生产构建成功

<!-- comet-task-ref:df0c8821-00ef-4567-ab6b-d38a52acf14d -->
### 4.3 实机音量验证

- 场景：开启功能（70%）；系统音量 50% → 录音中约 15% → 结束还原 50%；短按取消音量不变；录音中手动调节后结束保留用户值；Esc 中止等 fork 结束路径还原正常（决策 6 的核对）
- 验收：本机 CoreAudio 实测记录写入验证说明

<!-- comet-task-ref:dc6765d0-589e-4814-958f-5d3504525b0b -->
### 4.4 旧配置兼容冒烟

- 场景：settings.json 含 `volume: 15`，应用正常启动、功能默认关闭
- 验收：本机实测记录

<!-- comet-task-ref:4b726857-c250-452d-a466-aa5dbc8963da -->
### 4.5 fork 特性冒烟

- 场景：录音提示音播放、菜单栏图标状态、更新检查各一次
- 验收：本机实测记录

## 风险与回退

- 冲突误删 fork 功能：任务 1.2 逐文件核对兜底
- 全部失败回退：`git merge --abort`（合并中）或 revert merge commit（合并后）
