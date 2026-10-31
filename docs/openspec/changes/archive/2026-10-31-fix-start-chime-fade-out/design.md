# Design

## 修复方案（单方案）

把会话音量包络从「arm 预压低 → 确认还原 → 播开始音 → 播完再压低」改为「播开始音 → 播完单次压低 → 结束还原」。只删减 `app/src-tauri/src/controller.rs` 中的调用点，不新增逻辑、不改 `RecordingVolume`（crates/core）状态机。

### 变更点

1. `arm()`：删除开麦前的 `self.lower_volume()`；开麦失败分支的 `self.restore_volume()` 及注释一并删除（没有压低就无需还原）。
2. `start_recorder()` 预开麦转正分支：删除 `self.restore_volume()` 及「arm 时已压低音量，转正先还原」注释。
3. `disarm()`：删除 `self.restore_volume()` 及「取消预开麦，还原音量」注释。
4. 保留：`Msg::LowerVolume` 处理器中的压低（开始音播完后的唯一一次压低）、`stop_recording()` 中的还原。
5. 在 `Msg::LowerVolume` 与 `lower_volume()` 处的注释写明完整包络契约（正常音量播开始音 → 单次压低 → 结束还原；不得在开始音之前压低），防止回归。

### 为什么不采用其他做法

- **保留 arm 预压低、开始音改用不受系统音量影响的通道**：macOS 系统音量对 NSSound 与 CoreAudio 输出生效，绕过需要独立音频通道，改动远超 hotfix 范围。
- **确认时不还原、开始音在被压低的音量下播放**：即 `da38c63` 之前的旧行为，开始音明显偏轻，是用户已否定的体验。
- **压低改为渐变（fade）**：CoreAudio 无内建渐变，需要定时器多次写音量；用户本次反馈的是往返跳动而非阶跃感，不纳入本 change。

### 影响面

- 仅 `app/src-tauri/src/controller.rs` 一个文件；macOS 专属路径（`#[cfg(target_os = "macos")]`）。
- 不涉及设置项、前端、规格场景变更（`audio-feedback` spec 的「主流程不受影响」要求保持：提示音时序不变、失败静默降级不变）。
