# Brainstorm Summary

- Change: audio-feedback-sounds
- Date: 2026-10-31

## 确认的技术方案

（用户已于 2026-10-31 在 Step 1c 确认以下方案）

1. **播放引擎（沿用 Open design.md D1）**：Rust 侧新增 `app/src-tauri/src/sound.rs`，基于 rodio（底层复用项目已在用的 cpal 栈）播放；`OutputStream` 懒初始化常驻，`Sink` fire-and-forget 存入全局注册表并在每次播放时清理已结束项；设置页试听走新命令 `preview_sound(id)` 复用同一引擎。
2. **事件→提示音映射（代码核实后细化）**：
   - 开始：`start_recorder()` 成功路径统一触发（覆盖 begin/resume/begin_multi 与预开麦转正两条路径）；`resume()` 继续说新分段也发声（属于一次新的录音开始）。
   - 处理：`end()` 停止录音进入 wait 时触发一次（单候选与 Pick/Compare 统一）。
   - 完成：单候选=定稿有文本；Pick/Compare=全部行出结果（finished）时触发一次，面板保持等待用户挑选，挑选上屏不再额外发声。
   - 失败：转写超时（`abort(Some)`）、开麦失败（`start_recorder` Err）、单候选定稿无文本或出错、Pick 全部行失败时触发。
   - 不发声：用户取消（`abort(None)`/`cancel()`/`disarm()`）、arm 预开麦失败（会话尚未开始）、Pick/Compare 行级错误（会话仍在继续）。
3. **设置与资源（沿用 D2/D3）**：`Settings` 新增 `soundEnabled` + `soundStart/Processing/Done/Error`（扁平 camelCase，serde 默认兜底，sanitize 将未知音效 ID 回退默认）；WAV (PCM 16-bit) 音效放 `app/src-tauri/resources/sounds/`，`tauri.conf.json` 新增 `bundle.resources`；音效池约 6 个 CC0 素材（Kenney Interface Sounds 等），前端选项列表经新命令 `list_sounds()` 从 Rust 单一来源获取。
4. **设置 UI（沿用 proposal）**：Audio.svelte 新增「提示音」组：总开关行 + 四个事件行（音效下拉 + 旁边试听按钮），沿用 `.group/.row/save()` 惯例。

## 关键取舍与风险

- 隐藏 webview 播放不可靠 → Rust 播放与窗口状态解耦（候选方案已比较，见 design.md D1）。
- 开始音可能进入预开麦缓冲被识别（与 cometix-dictation 相同取舍）→ 短促音色缓解，不改变开麦时序。
- rodio 新依赖 → `default-features = false` 只启用 WAV 解码，控制编译体积；CI 构建（macOS+Windows）验证。
- 输出设备切换在应用运行中不跟随 → 可接受（重启生效）；无输出设备时静默跳过。
- 空识别结果按「失败」音处理（当前 HUD 显示"没有识别到内容"，属识别失败语义）。

## 测试策略

- Rust 单元测试：音效 ID 表完整性（每个 ID 有对应资源文件）、sanitize 未知 ID 回退、默认值兜底（旧配置 JSON 反序列化）。
- 手动走查：按 specs/audio-feedback/spec.md 场景清单（dev 模式逐场景听音 + 关开关对比行为不变）。
- 构建：本地 `npm run tauri build`（macOS）确认资源入包；Windows 由 CI 桌面构建覆盖。

## Spec Patch

无——已确认 spec 场景已覆盖上述映射（"多候选面板呈现结果"即全部行出结果的时点）。映射细节写入 Design Doc，不新增第二份需求规格。
