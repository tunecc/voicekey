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
