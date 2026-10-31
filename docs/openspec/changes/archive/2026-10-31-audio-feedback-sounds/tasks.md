# Tasks

## 1. 音效资源与打包

- [x] 1.1 从 CC0/公有领域素材库（如 Kenney UI 音频包）挑选 5–6 个短促候选音效，转为 WAV (PCM 16-bit) 放入 `app/src-tauri/resources/sounds/`，并随包新增授权与来源清单文件。验证：清单逐文件列出来源与授权；`git status` 仅包含预期新增文件。 <!-- comet-task:68d16b86-eb0a-4d5f-9fd4-9dca190621f7 -->
- [x] 1.2 在 `app/src-tauri/tauri.conf.json` 的 `bundle.resources` 登记音效目录，并在 Rust 侧实现运行时音效文件定位。验证：`npm run dev` 启动后日志可解析出音效文件路径且文件存在。 <!-- comet-task:b2b34c82-1ce5-4120-ad5c-d5781b0bf8e4 -->

## 2. 播放引擎与设置字段

- [x] 2.1 `app/src-tauri` 引入 rodio 依赖，新增 `src/sound.rs`：`SoundEvent` 枚举、懒初始化常驻输出流（`OnceLock`）、fire-and-forget 播放、播放失败静默（debug 日志、不向上传播）。验证：`cargo check` 通过；模块级单元测试覆盖事件→音效 ID 解析与开关关闭时跳过播放的分支。 <!-- comet-task:9a50e35e-8b87-4188-a69a-cdc54d748cba -->
- [x] 2.2 `Settings` 新增 `sound_enabled`（默认 true）与 `sound_start`/`sound_processing`/`sound_done`/`sound_error` 字段，补 `Default`；`sanitize()` 将未知音效 ID 回退为该事件默认值；同步 `app/src/lib/api.ts` 的 `Settings` interface。验证：单元测试用无新字段的旧配置 JSON 反序列化后得到默认值；未知 ID 经 sanitize 后回退。 <!-- comet-task:267b79dd-fa37-4fff-8917-27b13362cecf -->

## 3. 会话事件接线

- [x] 3.1 在 `controller.rs` 接入触发点：录音开始（`start_recorder` 成功与 `take_primed` 转正）、进入处理（`end()` 的 wait 转换，单候选与 Pick/Compare 统一触发一次）、完成（`done()` 成功定稿处，会话级一次）、失败（`done()` 错误/空结果分支与转写超时分支）；`cancel()`/`abort()` 不触发。验证：`npm run dev` 下手动走查——开始/结束/出结果/失败（可临时断网或超时）各发声一次、取消静音。 <!-- comet-task:db84e1f1-f649-4a70-9cc2-254a03013a35 -->
- [x] 3.2 确认提示音触发不改变会话状态机行为与时序：总开关关闭时各场景表现与现版本一致。验证：关闭开关后重复 3.1 走查；`cargo test` 全量通过。 <!-- comet-task:cd0b6b8d-f70b-4280-a4a2-196a308049ca -->

## 4. 设置界面与试听

- [x] 4.1 新增 Tauri 命令 `preview_sound(id)` 复用 `sound.rs` 播放引擎（不受总开关约束）。验证：`npm run dev` 下调用命令能听到对应音效。 <!-- comet-task:d33aa40a-534b-4eb8-98ff-25915a775ee3 -->
- [x] 4.2 `app/src/pages/Audio.svelte` 新增「提示音」设置组：总开关行 + 四个事件行（音效下拉 + 旁边试听按钮），沿用现有 `.group`/`.row`/`save()` 惯例；选择变更即时生效。验证：界面操作——切换开关、逐事件换音效、点击试听即时出声、重启应用后设置保留。 <!-- comet-task:34bed515-10d1-45da-9e0e-a837666e267f -->

## 5. 集成验证

- [x] 5.1 按规格走查 `specs/audio-feedback/spec.md` 全部场景（含缺省字段兜底、播放失败静默、行为时序不变），记录验收结果。验证：走查清单逐项通过。 <!-- comet-task:5481c12f-150d-4f88-99ee-8e5a126fd379 -->
- [x] 5.2 本地完成桌面构建（`npm run tauri build`，macOS），确认音效资源进入产物且安装包可正常播放；Windows 一致性交由 CI 桌面构建验证。验证：构建成功、产物含音效资源。 <!-- comet-task:cfda67bc-b8a9-4625-b05d-d3a86e292157 -->
