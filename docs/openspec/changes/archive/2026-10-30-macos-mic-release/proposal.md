# Proposal

## Why

macOS 上语音输入结束后，系统麦克风指示仍常驻，本进程持续占用麦克风；本机经常复现。核对源码后确认：`Recorder` 依赖录音线程 `drop(stream)` 来停止采集，但 cpal 0.15.3 的 CoreAudio 后端在**按名称选择的设备**上会注册断连监听，监听闭包持有 `Stream` 克隆，与 `StreamInner` 形成强引用环（`StreamInner → listener → Stream → Arc<StreamInner>`），且 `Stream` 没有 `Drop` 实现。于是 `StreamInner` 永不析构，其 `AudioUnit` 也永不停止：每次录音结束后输入单元仍在运行，麦克风一直被占用。只有走 `default_input_device()`（`is_default = true`）的设备不会注册该监听，因此默认设备用户不易复现。

另外，设置页/引导页的输入电平表在窗口失去焦点后仍继续采集，设置页停留在「音频」页并切到其他应用时，本轮语音结束后电平表会重新打开麦克风并长期占用。

上游 PR #7 的处理仅作参考；本修复按本仓库代码独立核对后重写，未采用其 `primed` 改动（核对我们所有取消路径都会经 `HotEvent::Cancel → disarm()` 释放预开麦，未见泄漏路径）。

## What Changes

- 录音线程收到停止信号后，先显式 `stream.pause()` 再 `drop(stream)`：即使 cpal 的引用环导致 `StreamInner` 泄漏，`AudioUnit` 也会停止，麦克风随之释放。
- 设备选择快路径：设置里选中的麦克风名称若就是系统默认输入设备，改用 `default_input_device()`，从源头避开 cpal 为非默认设备注册的断连监听与引用环。
- 输入电平表只在设置窗口聚焦时采集：失焦立即停止，聚焦恢复；关闭窗口时保持现有停止逻辑。
- 新增 macOS 实机回归测试（`#[ignore]`），断言 `Recorder::stop()` 后输入设备不再运行；另加一条 cpal 层测试固定「pause 后释放非默认设备」的行为。

## Capabilities

### New Capabilities

（无）

### Modified Capabilities

（无。修复现有缺陷，不改变 spec 验收场景。）

## Impact

- `crates/core/src/audio.rs`：设备选择与录音线程停止逻辑。
- `crates/core/src/audio/devices.rs`：测试辅助，查询设备运行状态。
- `app/src/pages/Audio.svelte`、`app/src/pages/Onboard.svelte`：电平表聚焦生命周期。
- 不涉及公共 API、数据库 schema、跨模块架构调整；不改变 Tap/LongPress 语义与录音数据通路。
