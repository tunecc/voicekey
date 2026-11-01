# Comet Design Handoff

- Change: sync-upstream-volume-duck
- Phase: design
- Mode: compact
- Context hash: cad91e2b3ec9679be918e3d0803ab119e08deb8fe8d7db6800cf1a8acf8fa7bc

Generated-by: comet-handoff.sh
Task hash policy: task-content-v1. Read tasks.md for live completion; excerpts are design-time context.

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## docs/openspec/changes/sync-upstream-volume-duck/proposal.md

- Source: docs/openspec/changes/sync-upstream-volume-duck/proposal.md
- Lines: 1-35
- SHA256: f9e96d3dccf10fc27f45eff19459cce2a47204c41f183d259052f677cb6621d6

```md
# Proposal

## Why

上游 J3n5en/voicekey 已通过 PR #8 合并「录音时降低系统音量」功能（按比例降低、默认关闭），并以四点实质缺陷为由关闭了本 fork 的 PR #10（固定值 15% 方案）：默认等于开启会改动所有升级用户的系统音量、固定值会把低音量用户调大、`arm()` 预开麦阶段就压低音量、与上游 main 冲突。fork 需要同步上游并切换到上游实现，避免维护一套已被维护者否决的第二实现。

## What Changes

- 合并 `upstream/main` 的 6 个提交（PR #8 音量 ducking、PR #7 macOS 麦克风修复、桌面 release CI 修复等），沿用 fork 惯例使用 merge commit
- **BREAKING** 移除 fork 的固定值音量实现，切换为上游实现：
  - 删除 `crates/core/src/audio/volume.rs`（`RecordingVolume`）及 `controller.rs` 中的 `lower_volume()`/`restore_volume()` 调用点
  - 删除 `Settings.volume` 字段（`settings.json` 的 `volume`），引入上游的 `lowerSystemVolume`（默认 `false`）与 `lowerSystemVolumePercent`（默认 70，范围 20–100）
  - `Audio.svelte` 的音量行改为上游的「录音时降低音量」UI（独立「系统音频」分组）
- 麦克风修复去重：上游 PR #7 与 fork 的 #1 修复是同一 bug（cpal 0.15 CoreAudio Stream 泄漏）的同一修法（默认设备走 `default_input_device` 快路径 + drop 前 `stream.pause()`）；合并时以上游 #7 为准，删除 fork 的重复实现，保留 fork 侧互补的 live 回归测试
- `README.md` 音量功能描述从「录音时调整音量（固定值）」更新为上游行为「录音时降低音量（按比例、默认关闭）」
- 不做 settings 迁移代码：fork 无已发布 release，无存量用户；旧 `settings.json` 中的 `volume` 字段被忽略，功能回到默认关闭，由用户手动重新开启

## Capabilities

### New Capabilities

- `recording-volume-ducking`: 录音期间按相对比例平滑降低系统输出音量、结束后安全还原；默认关闭；用户手动调节音量时让出控制权；处理输出设备切换与硬件音量量化；macOS 与 Windows 双平台

### Modified Capabilities

<!-- 现有 specs（app-update-channel、audio-feedback、hotkey-modes）不涉及音量行为，无需求级变更 -->

## Impact

- **删除**：`crates/core/src/audio/volume.rs`
- **新增（来自上游合并）**：`crates/platform/src/volume.rs`（`OutputVolumeDucker`）、`crates/platform/{mac.rs,win.rs}` 的音量后端、`Cargo.toml` 依赖
- **修改（冲突解决）**：`app/src-tauri/src/{controller.rs,main.rs,settings.rs}`、`app/src/lib/api.ts`、`app/src/pages/{Audio.svelte,Onboard.svelte}`、`crates/core/src/{audio.rs,audio/devices.rs}`、`Cargo.lock`
- **配置**：`.github/workflows/release.yml`（上游 CI 修复；fork 使用自有更新源，按现状取舍是否采纳）
- **文档**：`README.md`
- **用户可见行为**：升级后音量功能默认关闭；旧 `volume` 字段被忽略

```

## docs/openspec/changes/sync-upstream-volume-duck/design.md

- Source: docs/openspec/changes/sync-upstream-volume-duck/design.md
- Lines: 1-63
- SHA256: 047ac91f0db98e5d10250db70cf8845370b8ee6a9cfa12a8f475aaa2118f50d1

```md
# Design

## Context

fork 的 main = 上游 v1.7.1（merge-base `7bd0373`）+ 25 个 fork 提交；上游 main 在此之上有 6 个新提交（PR #8 音量 ducking、PR #7 麦克风修复、PR #12 CI 修复）。双方改动文件交集 9 个：`app/src-tauri/src/{controller.rs,main.rs,settings.rs}`、`app/src/lib/api.ts`、`app/src/pages/{Audio.svelte,Onboard.svelte}`、`Cargo.lock`、`crates/core/src/{audio.rs,audio/devices.rs}`。动机见 proposal.md「Why」；目标行为见 `specs/recording-volume-ducking/spec.md`。

关键事实（已核实）：

- 上游 #7（`6011a19`）与 fork #1（`860a735`）是同一 bug（cpal 0.15 CoreAudio Stream 与断连监听互相持有导致麦克风持续占用）的同一修法：选中设备即默认设备时走 `default_input_device` 快路径 + drop 前 `stream.pause()`。fork 侧另有 `devices::running()` 测试帮助函数与 live 回归测试。
- fork 的旧音量实现（`crates/core/src/audio/volume.rs` + `Settings.volume`）从未随任何 release 发布。
- `.github/workflows/release.yml` 仅上游改动，不在 fork 改动列表 → 无冲突。

## Goals / Non-Goals

**Goals:**

- main 分支获得上游 #8 的音量 ducking 行为与 #7 的麦克风修复，fork 特有功能零回归
- 冲突解决有逐文件依据，不留第二套重复实现
- 合并后 workspace 测试、clippy、前端构建全部通过

**Non-Goals:**

- 不对上游 #8 的行为做改进（有改进点后续对上游另提 PR）
- 不做 `volume` → `lowerSystemVolumePercent` 的配置迁移代码
- 不发版、不改版本号

## Decisions

1. **合并方式用 merge commit，不 rebase**。fork 历史惯例即 merge（`8b62adc`、`10a32a1`）；25 个本地提交 rebase 需重写已推送历史，成本与风险均高于收益。
2. **冲突取舍总原则：上游实现优先于 fork 的重复实现；fork 特有功能原样保留**。交集文件逐个策略：

   | 文件 | 取舍 |
   | --- | --- |
   | `settings.rs` | 音量字段采用上游（`lower_system_volume` + `lower_system_volume_percent`），删除 fork 的 `volume`；其余字段两边并集 |
   | `controller.rs` | 音量调用点采用上游（`OutputVolumeDucker` + `Msg::VolumeDucking` + `stop_recording` 单点还原）；fork 的提示音/菜单栏等多渠道逻辑保留 |
   | `main.rs` | 采纳上游的 meter/生命周期清理改动；fork 的初始化差异保留 |
   | `Audio.svelte` / `Onboard.svelte` | 音量 UI 行采用上游；fork 的样式与提示音相关行保留 |
   | `api.ts` | 设置接口字段采用上游定义 |
   | `crates/core/src/audio.rs` | `pub mod volume;` 声明删除；麦克风修复以上游文本为准 |
   | `crates/core/src/audio/devices.rs` | 以上游 #7 为准；fork 的 `running()` 帮助函数与 live 测试若上游没有则保留 |
   | `Cargo.lock` | 合并后由 cargo 重新生成并校验 |

3. **旧音量实现整体删除，不做兼容层**。`RecordingVolume`、`Settings.volume`、`Audio.svelte` 旧行、`lower_volume()/restore_volume()` 调用点全部移除；serde 默认忽略未知字段，旧 `settings.json` 直接兼容。理由：实现从未发布，兼容层只服务维护者本机一次。
4. **上游 CI 修复（release.yml）直接采纳**：fork 未改动过该文件，无合并成本；fork 发版流程如与「always Latest」冲突，届时再调整（属发版 change 范围）。
5. **验证以行为为准**：单元/集成测试（workspace）+ 前端构建 + 实机音量三段验证（开启→压低→还原）+ 旧 settings.json 加载冒烟。fork 特性（录音提示音、菜单栏图标、更新检查）各做一次手动冒烟。

## Risks / Trade-offs

- [冲突解决误删 fork 特有功能] → 逐文件对照 fork 改动清单核对；合并后对 fork 特性逐项冒烟
- [两份麦克风修复存在细微差异，取上游后引入回归] → 保留 fork 的 live 回归测试并实际跑通；若 fork 测试与上游实现冲突，以 spec 的可观测行为裁决
- [上游 `OutputVolumeDucker` 的 Windows 路径本机无法实测] → 与上游合并态一致，Windows 由上游 CI（`x86_64-pc-windows-msvc` check）覆盖，本机仅静态确认 cfg 门控
- [维护者本机旧 `volume: 15` 设置静默失效] → 接受：spec 已声明该字段被忽略，升级后手动重新开启即可

## Migration Plan

1. `git merge upstream/main`（merge commit），按上表解决冲突
2. 删除旧音量实现，修复编译
3. 全量验证（见 Decisions 5）
4. 回滚策略：revert 该 merge commit 即回到同步前状态，无数据迁移

## Open Questions

无。

```

## docs/openspec/changes/sync-upstream-volume-duck/tasks.md

- Source: docs/openspec/changes/sync-upstream-volume-duck/tasks.md
- Lines: 1-29
- SHA256: 2e466d90143b641f60fe68313ce860becb255a291102c1be91b4248936bf712b

```md
# Tasks

## 1. 合并上游 main

- [ ] 1.1 执行 `git fetch upstream && git merge upstream/main`，冲突逐个按 design.md 决策表处理，产出 merge commit。验证：`git status` 干净且 merge commit 在 main 历史中 <!-- comet-task:7a133d0a-07f7-46d5-b7fc-bbf0c7a31419 -->
- [ ] 1.2 对照 9 个交集文件（design.md 决策表）逐文件核对取舍：音量相关采用上游，fork 特有功能（录音提示音、菜单栏图标、自有更新源）代码仍在。验证：逐文件 `git diff` 审查记录 + grep 确认 fork 特性代码存在 <!-- comet-task:3c56499b-f56c-4c08-b91c-9486101ca74e -->

## 2. 切换音量实现并清理旧实现

- [ ] 2.1 删除 `crates/core/src/audio/volume.rs` 与 `crates/core/src/audio.rs` 中的 `pub mod volume;`，移除 `controller.rs` 中 `lower_volume()`/`restore_volume()` 及全部调用点。验证：`cargo check -p voicekey-core` 通过，仓库内 `grep -r RecordingVolume` 无结果 <!-- comet-task:d9cafde0-336f-4abc-b801-92c72b1e0ffb -->
- [ ] 2.2 `app/src-tauri/src/settings.rs` 删除 `volume` 字段、采纳上游 `lower_system_volume`/`lower_system_volume_percent`，`app/src/lib/api.ts` 对齐上游字段。验证：`cargo check --workspace` 通过 <!-- comet-task:da347451-03e0-4f19-ab38-4e2876bdafdf -->
- [ ] 2.3 `app/src/pages/Audio.svelte` 音量行替换为上游「录音时降低音量」UI（系统音频分组）。验证：`npm run build` 通过 <!-- comet-task:27d6167d-be27-4cf6-ad32-c00d6ed05eee -->
- [ ] 2.4 `README.md` 音量功能描述更新为上游行为（按比例降低、默认关闭）。验证：README 中不再有固定值 15% 描述 <!-- comet-task:93bd4e73-520b-4f14-8efc-52a9701854c6 -->

## 3. 麦克风修复去重

- [ ] 3.1 `crates/core/src/audio.rs` 与 `crates/core/src/audio/devices.rs` 以上游 #7（`6011a19`）文本为准合并，删除 fork #1 的重复实现，保留 fork 的 `devices::running()` 帮助函数与 live 测试。验证：`cargo test -p voicekey-core` 通过 <!-- comet-task:28a1c26a-bc26-488b-b132-be6eae577348 -->

## 4. 验证

- [ ] 4.1 `cargo test --workspace` 与 `cargo clippy --workspace` 全部通过。验证：命令退出码 0 <!-- comet-task:e9fc7136-34ed-483d-837b-b311dac42835 -->
- [ ] 4.2 `npm ci && npm run build` 通过。验证：前端生产构建成功 <!-- comet-task:07375266-9a0f-4b9e-8041-d05055fda909 -->
- [ ] 4.3 实机音量验证：开启功能（比例 70%），系统音量 50% → 录音中约 15% → 结束还原 50%；短按取消音量不变；录音中手动调节后结束保留用户值。验证：本机 CoreAudio 实测记录 <!-- comet-task:df0c8821-00ef-4567-ab6b-d38a52acf14d -->
- [ ] 4.4 旧配置兼容冒烟：settings.json 含 `volume: 15` 时应用正常启动、功能默认关闭。验证：本机实测记录 <!-- comet-task:dc6765d0-589e-4814-958f-5d3504525b0b -->
- [ ] 4.5 fork 特性冒烟：录音提示音、菜单栏图标、更新检查各验证一次正常。验证：本机实测记录 <!-- comet-task:4b726857-c250-452d-a466-aa5dbc8963da -->

## Workflow follow-up

- 归档与交付（提交、push 到 origin main）按 `/comet-archive` 的确认流程执行，不在本任务清单内跟踪

```

## docs/openspec/changes/sync-upstream-volume-duck/.openspec.yaml

- Source: docs/openspec/changes/sync-upstream-volume-duck/.openspec.yaml
- Lines: 1-2
- SHA256: 108f7b3dae33488206672f31b510275110c0194c93062eb1970e19ca60c08722

```md
schema: spec-driven
created: 2026-11-01

```

## docs/openspec/changes/sync-upstream-volume-duck/specs/recording-volume-ducking/spec.md

- Source: docs/openspec/changes/sync-upstream-volume-duck/specs/recording-volume-ducking/spec.md
- Lines: 1-88
- SHA256: a7695857b0fcb7261212db25993fa12ebf3e87b6ca8fda3c03dcea21c3b6adb8

[TRUNCATED]

```md
# Spec Delta

## Purpose

在录音期间按相对比例降低系统输出音量以减少串音干扰，结束后安全还原；功能默认关闭，用户对系统音量的手动控制始终优先于自动降低。

## ADDED Requirements

### Requirement: 功能默认关闭
录音时降低系统输出音量的功能 SHALL 默认关闭；升级或全新安装后，录音行为 SHALL 与该功能不存在时完全一致。设置中 SHALL 提供独立开关与降低比例（20%–100%，步长 5%，默认 70%，100% 显示为「静音」）。

#### Scenario: 默认设置下录音
- **WHEN** 用户未开启该功能时开始并结束录音
- **THEN** 系统输出音量全程不变

#### Scenario: 旧配置文件升级
- **WHEN** 已存在的 settings.json 含有本功能不认识的旧字段（如 `volume`）
- **THEN** 应用正常启动加载，该字段被忽略，功能保持默认关闭

### Requirement: 按相对比例降低且只降不升
功能启用且确认开始录音后，系统 SHALL 以录音开始时的实际输出音量为基线，按「最终保留 = 原音量 × (1 − 降低比例)」压低输出音量；任何情况下 SHALL NOT 将音量调高于基线。

#### Scenario: 常规降低
- **WHEN** 录音开始时输出音量为 50%，降低比例为 70%
- **THEN** 录音期间输出音量降至 15%

#### Scenario: 低音量不被调大
- **WHEN** 录音开始时输出音量为 2%，降低比例为 70%
- **THEN** 录音期间输出音量不高于 2%

### Requirement: 仅在确认录音后生效
音量降低 SHALL 仅在确认真正进入录音状态后开始；快捷键按下、预开麦（preroll）阶段 SHALL NOT 改变系统音量；误触、短按取消 SHALL NOT 影响系统音量。

#### Scenario: 短按取消
- **WHEN** 用户按下快捷键后立即松开（未进入录音）
- **THEN** 系统输出音量不变

#### Scenario: 确认开始录音
- **WHEN** 用户长按快捷键确认进入录音
- **THEN** 输出音量开始按比例降低

### Requirement: 平滑过渡且基线稳定
降低与还原 SHALL 为平滑过渡而非硬切；录音中修改降低比例 SHALL 从当前实际音量平滑重定向到新目标；连续快速开始/结束录音 SHALL NOT 把过渡中间值误当新基线， SHALL NOT 出现逐轮递减。

#### Scenario: 开始录音平滑降低
- **WHEN** 确认开始录音
- **THEN** 输出音量在约四分之一秒内平滑过渡到目标值，无跳变

#### Scenario: 录音中调整比例
- **WHEN** 录音期间将降低比例从 70% 调整为 40%
- **THEN** 音量从当前实际值平滑过渡到新目标，基线仍为本次录音开始时的原始音量

#### Scenario: 连续两轮录音
- **WHEN** 第一轮录音结束后立即开始第二轮
- **THEN** 第二轮以用户当前音量为基线，两轮结束后音量还原为第一轮前的原始值

### Requirement: 用户手动调节优先
录音期间用户手动调节系统音量后，自动降低 SHALL 立即停止干预并保留用户设置；恢复过程中用户再次手动调节 SHALL 保留用户的新音量；手动让权后调整降低比例 SHALL NOT 重新接管音量。录音开始时设备处于静音或音量为 0 的，本轮 SHALL NOT 接管。

#### Scenario: 录音中手动调低音量
- **WHEN** 录音期间用户手动将音量调至其他值
- **THEN** 自动降低停止，录音结束后保留用户设置的音量

#### Scenario: 还原途中手动调节
- **WHEN** 录音结束、还原过程中用户手动调整音量
- **THEN** 自动还原停止，保留用户的新音量

#### Scenario: 起始音量为零
- **WHEN** 录音开始时输出音量为 0 或设备静音
- **THEN** 本轮不改变音量；录音期间用户主动调高后也不会被自动压低

### Requirement: 输出设备切换与还原安全
录音期间切换默认输出设备时，系统 SHALL 只尝试恢复旧设备自身的音量，新设备 SHALL 以自己的当前音量建立独立基线后再降低；旧设备已断开或状态无法确认时 SHALL 放弃恢复，SHALL NOT 将旧设备音量写入其他设备。

#### Scenario: 录音中切换输出设备
- **WHEN** 录音期间用户切换默认输出设备
- **THEN** 旧设备音量被还原，新设备按自身音量为基线继续降低

#### Scenario: 旧设备已断开
- **WHEN** 录音结束后旧输出设备已断开

```

Full source: docs/openspec/changes/sync-upstream-volume-duck/specs/recording-volume-ducking/spec.md
