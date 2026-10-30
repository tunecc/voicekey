# macos-mic-release 验证报告

- 日期：2026-10-30
- Change：macos-mic-release（workflow: hotfix）
- 验证模式：light（`comet state scale` 因任务数为 5 建议 full；本 change 无 delta spec、改动 4 个文件、不涉及公共 API/schema，风险由实机测试覆盖，保留已选 light）
- 结论：PASS

## 验证环境

- macOS（本机），默认输入设备 MacBook Pro Microphone；本轮实机测试使用默认设备与空闲的非默认设备（iPhone (4) Microphone）。
- 实机测试以 `kAudioDevicePropertyDeviceIsRunning` 断言设备是否仍被本进程占用，等价于系统麦克风指示的触发条件。

## 轻量验证 7 项

| # | 检查项 | 结果 | 证据 |
| - | ------ | ---- | ---- |
| 1 | tasks.md 全部完成 | PASS | 5/5 已勾选（`docs/openspec/changes/macos-mic-release/tasks.md`） |
| 2 | 改动文件与 tasks 一致 | PASS | `crates/core/src/audio.rs`（停止前 pause、默认设备快路径、实机测试）、`crates/core/src/audio/devices.rs`（测试辅助）、`app/src/pages/Audio.svelte`、`app/src/pages/Onboard.svelte`（电平表聚焦生命周期） |
| 3 | 编译通过 | PASS | `cargo test -p voicekey-core --lib -- --include-ignored` 编译成功；`npm --prefix app run build`（vite build）成功，148 modules transformed |
| 4 | 相关测试通过 | PASS | 修复前 RED：两条实机测试均以 `left: Some(1)`（停止后设备仍在运行）失败；修复后 GREEN：`test result: ok. 24 passed; 0 failed; 0 ignored`（含 3 条原先 `#[ignore]` 的实机测试） |
| 5 | 无明显安全问题 | PASS | 无硬编码密钥；新增 `unsafe` 仅位于 `#[cfg(test)]` 的设备状态读取辅助，先校验返回字节长度，与既有 `property()` 读取模式一致；不新增外部输入处理 |
| 6 | 最终集成代码审查 | PASS（跳过自动审查） | `review_mode: off`（hotfix 预设默认值），按规则记录原因：改动集中在单一录音停止路径与两个页面的电平表开关，已由实机测试覆盖核心行为 |
| 7 | 核心成功/失败/边界场景 | PASS | 成功：默认设备具名录音 stop 后设备不再运行；失败路径：非默认设备 stop 后同样释放；边界：零通道设备延迟可见、默认设备切换/断开快照、输出设备变化不误触发（既有单测）；电平表聚焦改动经前端构建与代码核对，真实窗口的 focus/blur 行为未做 UI 自动化，列为人工复看项 |

## 手工复核说明

- 主因（语音结束后麦克风占用）已通过实机测试以设备运行状态直接验证，未依赖菜单栏指示的人工观察。
- 次因（设置页失焦后电平表停止采集）属于前端窗口事件行为，本环境无法自动化 Tauri 窗口 UI；建议日常使用时确认：设置停留在「音频」页并切到其他应用后系统麦克风指示消失，回到窗口后电平表恢复。

## 执行命令

```bash
cargo test -p voicekey-core --lib -- --include-ignored --test-threads=1
npm --prefix app run build
```
