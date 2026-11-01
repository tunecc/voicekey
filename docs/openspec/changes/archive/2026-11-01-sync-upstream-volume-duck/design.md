---
comet_change: sync-upstream-volume-duck
role: technical-design
canonical_spec: openspec
archived-with: 2026-11-01-sync-upstream-volume-duck
status: final
---

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
6. **fork 结束路径的还原集成规则**：上游在 `stop_recording` 单点还原音量（`set_active(false)`）；合并后核对 fork 的全部录音结束路径（Esc 中止、多渠道挑选完成、取消预开麦等），发现绕过上游还原点的路径时，把上游的还原调用补挂到该路径，保持上游单点风格，不恢复 fork 旧的逐路径 `restore_volume()` 模式。

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
