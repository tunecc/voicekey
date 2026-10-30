# Design

## 根因

1. **cpal 强引用环（主因）**
   - `cpal 0.15.3` 的 `Stream` 是 `Arc<Mutex<StreamInner>>` 的克隆，没有 `Drop`。
   - `build_input_stream_raw` 仅在 `!device.is_default` 时调用 `add_disconnect_listener`；该函数把 `Stream` 克隆捕获进闭包，闭包存入 `StreamInner._disconnect_listener`，构成 `StreamInner → listener → Stream → Arc<StreamInner>` 强引用环。
   - 因此录音线程 `drop(stream)` 只减少外部计数，`StreamInner` 与 `AudioUnit` 永不析构；`audio_unit.stop()`（`AudioOutputUnitStop`）从未执行，采集保持运行。
   - 本仓库 `Recorder::start` 对「按名称选择的设备」走 `selected_device()`，其设备来自 `host.devices()`，`is_default` 恒为 `false`，必然注册断连监听；只有空选择（系统默认）才走 `default_input_device()` 而免疫。
2. **电平表生命周期（次因）**
   - `Audio.svelte` / `Onboard.svelte` 挂载时开启电平表；窗口失焦/退到后台后仍持续采集，设置页停留在「音频」页时每轮语音结束都会重新开麦。

## 修复方案

### 1. `Recorder` 停止时先暂停（crates/core/src/audio.rs）

录音线程等待停止信号后：

```rust
let _ = stop_rx.recv();
let _ = stream.pause(); // cpal 0.15 的 CoreAudio Stream 有引用环，drop 不会停止音频单元
drop(stream);
```

`Stream::pause()` 调用 `AudioUnit::stop()`（`AudioOutputUnitStop`），停止 IOProc，释放麦克风。`StreamInner` 仍会泄漏（上游 bug，注释标注），但不再占用设备。

### 2. 默认设备快路径（crates/core/src/audio.rs）

`selected_device()` 先比较选择名与 `default_input_device()` 名称；一致时直接返回默认设备，避免注册断连监听。名称不匹配时保持现有 `named_input` 逻辑。

### 3. 电平表只在聚焦时采集（Audio.svelte / Onboard.svelte）

- 监听 `window` 的 `focus` / `blur`，用 `document.hasFocus()` 同步 `meter(on)`；卸载时照旧 `meter(false)`。
- 引导页额外保持「仅 mic 步骤开表」的现有条件。

### 4. 回归测试（audio.rs + devices.rs 测试辅助）

- `devices.rs` 增加 `#[cfg(test)]` 辅助：默认输入设备 ID 与设备运行状态读取（`kAudioDevicePropertyDeviceIsRunning`）。
- `audio.rs` 增加两条 `#[ignore]` 实机测试：
  1. `Recorder::start(Some(默认设备名))` 后设备运行；`stop()` 后设备不再运行（当前实现会失败）。
  2. cpal 层：从 `host.devices()` 取默认设备（非默认路径）建流，`pause()` 后释放。

## 边界与风险

- 非默认设备仍会泄漏 `StreamInner`（每轮录音一个小对象与断连监听），但不再占用麦克风；彻底修复需等待上游 cpal。
- `pause()` 失败仅记录日志，不阻断停止流程。
- 不改 `primed`/`cancel()`：核对所有取消路径均经 `HotEvent::Cancel → disarm()` 释放，未发现泄漏路径。
- 前端聚焦切换只影响电平表，不影响录音会话。

## 验证

- `cargo test -p voicekey-core --lib`：现有单测通过。
- `cargo test -p voicekey-core --lib -- --ignored live_`：两条实机测试通过（修复前第 1 条应失败）。
- 前端 `npm run build`（app 目录）通过。
- 手动：选择具名麦克风说一句，结束后系统麦克风指示消失；设置页停在「音频」页切到其他应用，指示消失，回到窗口恢复。
