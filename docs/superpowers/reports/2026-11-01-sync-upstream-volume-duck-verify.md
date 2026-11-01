# 验证报告：sync-upstream-volume-duck

- 日期：2026-11-01
- 模式：full（规模评估：12 任务 / 1 delta spec / 31 变更文件）
- 基线：6144e16（merge-base 7bd0373 上游 v1.7.1）→ HEAD（merge commit + 产物提交）

## Summary

| 维度 | 状态 |
| --- | --- |
| Completeness | 12/12 任务完成；1 个新 capability spec（7 需求 / 13 场景）全部有实现对应 |
| Correctness | workspace 测试全绿（61 passed / 4 live ignored）；核心场景用户实机验证通过 |
| Coherence | 冲突解决与 design.md 决策表一致；delta spec 与 design 无矛盾（无 Spec Patch） |

## 检查项结果（full）

| # | 检查项 | 结果 | 证据 |
| --- | --- | --- | --- |
| 1 | tasks.md 全部完成 | PASS | comet task-complete 12/12（revision 2e466d90） |
| 2 | 改动与 tasks 一致 | PASS | merge commit 4809e8e + docs 提交；9 交集文件 + volume.rs 删除 + README，与决策表逐项对应 |
| 3 | 编译通过 | PASS | cargo check --workspace 0 error；build 守卫自动执行 cargo 构建通过（phase→verify） |
| 4 | 相关测试通过 | PASS | cargo test --workspace：36(controller)+9(core)+14(platform 音量状态机)+1 全部通过，4 项 live 硬件测试按设计 ignore |
| 5 | 无安全问题 | PASS | 无硬编码密钥；无新增 unsafe（见独立审查报告） |
| 6 | 最终集成审查 | PASS | 独立 reviewer 审查最终 diff，结论可归档（见下方审查摘要） |
| 7 | 核心与边界场景 | PASS | 见下方场景覆盖 |

## 场景覆盖（specs/recording-volume-ducking）

- 默认关闭 / 旧字段忽略：serde 忽略未知字段 + 用户实机验证（注入 volume:15 启动正常）
- 按比例只降不升 / 基线稳定 / 量化硬件：platform 14 项状态机测试（上游随 #8 合入，全绿）
- 仅确认录音后生效（arm 不压低）：上游实现 + start_recorder 集成点核对（arm 无音量调用）
- 用户手动调节优先：同上（DuckState yielded 逻辑测试）
- 实机交互场景（降低/还原/短按/手动调节）：**用户实机验证 5 项全部通过**（debug 构建，2026-11-01）
- fork 提示音时序（开始音播完后再压低）：**打包产物实测**（tauri build 产物，见附录）

## 与 Build 的差异处理

- Build 遗留「提示音×压低时序需打包产物验证」→ 本阶段以 `npx tauri build` 打包产物完成实测，结果见附录
- mac.rs clippy 告警（unnecessary_mut_passed 等）为上游 PR #8 原代码自带，非本次引入，接受
- sound.rs unused import 告警为 fork 既有问题，范围外，接受

## 独立审查摘要

独立 reviewer 对 6144e16..HEAD 与两个父提交三方对照审查，并独立实跑全部验证命令：

- **CRITICAL / IMPORTANT：无**
- **结论：可以进入归档**（fork 改动与上游代码边界干净，平台文件与上游逐字节一致，结束路径还原与上游单点完全同构）
- W1（已修复）：fork `selected_device` 默认设备快路径为死代码（上游 `Recorder::start` 拦截臂先行覆盖），与 tasks 3.1「删除重复实现」声明不符 → 第 2 轮 build 已删除
- W2（记录）：tauri.conf 版本号 1.7.1→1.8.0 系合并继承上游 `c7e876d v1.8.0`，非本 change 发版意图；fork 跟随上游版本号惯例一致，归档记录确认
- W3（记录，上游继承）：进程强退时音量可能滞留压低态——上游 #8 原设计（volume.rs 与上游逐字节一致），如需修复应对上游提 PR
- S1（接受）：延迟 duck 消息存在约一个提示音时长（≤440ms）的陈旧窗口，需 <0.5s 内取消+重触才可能触发且自愈；可选加固为会话令牌，作为后续上游改进项
- S2（已修复）：Audio.svelte 死 CSS（旧 `.val` 与 `input[type=range]` 宽度覆盖）→ 第 2 轮 build 已删除
- S3（接受）：clippy 告警均归属上游新代码或 fork 未触碰文件；本次改动文件零告警

## 验证失败与修复轮次

- 第 1 次 verify-fail（自动修复，上限 5）：审查发现 W1/S2 两处局部问题 → build 删除死代码与死 CSS → core 测试 9 passed、前端构建通过、workspace 编译 0 error → build 守卫重过（cargo 10s）→ 重新进入 verify

## 附录：打包产物实测

- 产物：`target/release/bundle/macos/VoiceKey.app`（10.93 MiB，`npx tauri build`，2026-11-01）
- **提示音×压低时序：用户实机验证通过**——开始提示音以正常音量完整播放，播完后音量才平滑压低（fork「先播音、后压低」时序保持）；结束后正常还原
