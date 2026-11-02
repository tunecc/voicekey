# Design

修复 PR #17 review 的 7 个问题。以下按 review 编号给出实现要点（无方案对比——review 已限定合法选项，此处记录取舍理由与关键细节）。

## D1 移除「识别完成」行（review 1）

review 给出「去掉这一行」或「真正接线」两个合法选项。选择移除：PR 自身已论证「识别很快，完成信号由处理音承担」，接线会在处理音之后短间隔再响一声（听感重叠）；fork 侧 remove-custom-sound-scheme 提案是同一结论。

- `Audio.svelte`：`CHIME_ROWS` 删「识别完成」行 → 3 行
- `sound.rs`：删 `SoundEvent::Done` 变体（连带 `#[allow(dead_code)]`）、`event_id`/`play_duration` 对应分支、`DEFAULT_DONE`
- `settings.rs`：删 `sound_done` 字段、Default 对应项、sanitize 循环中的对应元组
- `api.ts`：`Settings` 接口删 `soundDone`
- 测试：`settings.rs` 的 none 保留测试字段表、legacy 回退测试断言同步
- 兼容：`#[serde(default)]` 下旧配置的 `soundDone` 键被静默忽略，下次保存自然消失

## D2 恢复窗口等待按「确实压低过音量」门控（review 2）

`controller.rs` 增加字段 `ducked: bool`（会话级）：

- `Msg::VolumeDucking` handler（controller.rs:303）：`enabled && self.recording` 时置 `ducked = true`（ducking 真正生效的唯一入口）；`enabled == false` 时清 `ducked = false`（音量恢复已在当时发起，stop 后无需再等）
- `start()`：新会话开始时清 `ducked = false`
- `cancel()`：清 `ducked = false`
- `play_after_restore`：`let wait = if self.ducked { self.restore_wait() } else { Duration::ZERO };`——未压过音量立即播放，识别快时字上屏与处理音不再脱节

## D3 非 macOS 只列有兜底音频的音效（review 3）

- `sound.rs` 新增 `pub fn selectable_sounds() -> &'static [&'static str]`：macOS 返回 `SYSTEM_SOUNDS` 全集；非 macOS 返回 `wav_fallback_id` 命中的子集（Tink/Pop/Basso）
- `main.rs` `list_sounds` 改用 `selectable_sounds()`（前端零改动，渲染返回列表）
- `settings.rs` sanitize 逐事件校验平台化：非法判定从 `!is_known(id)` 改为「非 macOS 且无兜底且非 none」→ 回退该事件默认；Windows 上历史配置里的 Glass 等自愈为默认音，不再出现「选了不响」
- 单测用 `#[cfg(not(target_os = "macos"))]`/`#[cfg(target_os = "macos")]` 分别断言列表内容

## D4 `soundEnabled` 默认 false（review 4）

- `settings.rs` Default：`sound_enabled: true` → `false`
- `legacy_config_without_sound_fields_falls_back_to_defaults` 断言改为 false，注释同步「缺省键=关，升级用户不出声」
- 不做「首次创建配置写 true」：避免首启写盘与缺省语义分叉；功能可在音频设置页一键打开

## D5 `cancel()` 作废排队提示音（review 5）

`cancel()` 开头加 `self.chime_gen.fetch_add(1, Ordering::Relaxed);`。`abort()` 现有顺序天然正确：先 `cancel()`（作废旧排队处理音），后 `play_after_restore(SoundEvent::Error)`（捕获递增后的新代数）→ 失败音不受影响。用户取消（message=None）保持安静，且不再响已排队的处理音。

## D6 延迟压音量消息带会话代数（review 6）

`start()` 中延迟发送 `VolumeDucking` 的 spawn：捕获 `my_gen = chime_gen.load()`，`sleep(delay)` 后校验 `gen.load() == my_gen` 才 `tx.send(...)`。与 `play_after_restore` 同一门闩模式；消除「上次结束后极短时间内再开始，旧延迟消息压到新会话开始音」。

## D7 资源收敛（review 7）

- 删除 7 个未使用 WAV：`confirmation/ding/glass/minimize/pluck/soft-alert/switch.wav`
- 保留 3 个：`tink.wav`（Tink 兜底·默认开始音）、`pop.wav`（Pop 兜底·默认处理音）、`error.wav`（Basso 兜底·可选失败音）——与 `wav_fallback_id` 映射完全一致
- `LICENSE.md`：表格重写为 3 行，删除旧「音效方案」时代的默认用途与叙述；来源与 CC0 授权段保留
- `tauri.conf.json` 的 `bundle.resources: resources/sounds/*` 通配不变，打包自动收窄

## 验证策略

- `cargo +nightly check -p voicekey`（0 警告）与 `cargo +nightly test --workspace`：上游基线 stable ≥1.95 无法编译 wtlocal（stdarch_neon_dotprod，项目记忆已验证），统一用 nightly
- `npm run build`（vite）
- macOS 打包实机试听（用户配合）：关 ducking 时处理音即时、Esc 取消后无排队音、快速结束→再开始无错拍、设置组 3 行
- Windows：代码级保证（cfg 编译分支 + 单测），PR 附注说明未实机验证
