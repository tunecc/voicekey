# Tasks

## 1. 音效素材检索与入库

- [x] 1.1 上网检索并挑选 4 个 CC0/可再分发音效：tink 类轻巧短音（开始）、pop 类轻弹音（处理）、轻快确认音（完成）、低调柔和短音（失败），时长 ≤0.3s、与现有音色协调不突兀；记录每个候选的来源页与授权 — 验证：4 个原始音频文件到手，来源与授权信息可追溯 <!-- comet-task:ec7930d8-6234-4370-a4d7-66ee14299f80 -->
- [x] 1.2 按现有规范（ffmpeg 裁首尾静音、mono、44.1kHz/16-bit PCM WAV）转码并以稳定 id 命名放入 `app/src-tauri/resources/sounds/`，在 `LICENSE.md` 增补来源、原始文件名、时长与授权清单 — 验证：`file` 确认 WAV 格式与参数，LICENSE.md 含全部新条目 <!-- comet-task:03fa2e7e-5fd3-48d6-a7a3-b296bbf0cdf5 -->
- [x] 1.3 `sound.rs` 的 `SOUNDS` 池登记新音效（id、中文名） — 验证：`cargo test sound_pool_is_not_empty_and_ids_match_wav_files` 通过 <!-- comet-task:120a36dc-fc9a-4525-99c4-6015b6a46af3 -->

## 2. Rust 侧方案门控

- [x] 2.1 `settings.rs` 新增 `sound_scheme` 字段（默认 `"custom"`、serde default 兜底、sanitize 校验非法值回退），`app/src/lib/api.ts` 镜像字段 — 验证：`cargo test` 新增用例通过（缺字段兜底、非法值回退） <!-- comet-task:35e24cc4-c508-4ece-8632-c3748bb2b0a1 -->
- [x] 2.2 `sound.rs` 新增简洁方案固定映射常量表与 `gated_id` 门控分支：`simple` 按映射取音效 id，`custom` 沿用逐事件字段，总开关逻辑不变 — 验证：`cargo test` 新增用例通过（simple 四事件按映射发声、custom 行为不变、总开关关闭全静音） <!-- comet-task:f0265e22-2f91-4972-b391-2605efe0eec5 -->
- [x] 2.3 确认方案切换不写逐事件字段：映射为常量读取，设置保存路径仅新增 scheme 字段 — 验证：单测断言 simple 门控只读不改写四个逐事件字段 <!-- comet-task:1558469e-773d-4b92-80b0-bcf9d2314113 -->

## 3. Tauri 命令

- [x] 3.1 `main.rs` 新增并注册 `simple_scheme()` 命令，返回四事件固定映射（事件键、事件标签、soundId、soundLabel），数据由 Rust 常量表生成 — 验证：调用命令返回正确映射，且每个 soundId 均在 `SOUNDS` 池内 <!-- comet-task:0a777fbe-8264-446b-ab92-c8b76e811e42 -->

## 4. 前端设置页

- [x] 4.1 `api.ts` 新增 `simple_scheme()` 调用封装与 `soundScheme` 设置字段镜像 — 验证：前端构建/lint 通过 <!-- comet-task:5ded6c17-7955-49dc-835b-1291db082d2b -->
- [x] 4.2 `Audio.svelte` 提示音组新增「音效方案」单选行（简洁/自定义），变更经 `save({ soundScheme })` 即时持久化 — 验证：切换后 `settings.json` 中 `soundScheme` 更新，重启应用后保持 <!-- comet-task:af744057-e00b-4c16-913d-bd212b744ced -->
- [x] 4.3 简洁方案下渲染只读固定映射行（事件名 + 音效名 + 试听按钮）并隐藏逐事件选择；切回自定义恢复逐事件行且保留原有选择 — 验证：界面切换核对两种形态；试听按钮播放当前方案下该事件实际音效 <!-- comet-task:ca8a65a3-fca5-46dd-b106-710e75105faa -->

## 5. 集成验证

- [x] 5.1 按 `specs/audio-feedback/spec.md` delta 场景逐条端到端验收：方案切换即时生效、简洁固定映射发声、自定义逐事件发声、取消静音、总开关全静音、试听行为、设置持久化、缺省字段兜底 — 验证：逐场景手动验收并记录结果 <!-- comet-task:d703345d-8558-41ad-a2ea-0b1128ff3c41 -->
- [x] 5.2 回归默认路径：使用不含 `soundScheme` 字段的旧 `settings.json` 启动，确认行为与升级前一致（默认自定义方案） — 验证：旧配置启动核对逐事件行为与持久化 <!-- comet-task:625b9c47-7031-418c-b55e-32ac33bf62dd -->

## 6. 纯双音复刻（走查后范围修订，用户已确认）

- [x] 6.1 Rust 侧纯双音与 macOS 系统音：`SIMPLE_SCHEME` 收敛为开始/处理两映射（完成/失败在简洁方案下不发声）；macOS 经 `objc2-app-kit` NSSound 在主线程播系统音 Tink/Pop（不可用时回退内置近似音），Windows 直接用内置近似音；试听按当前方案与平台解析实际音源 — 验证：`cargo test` 新增用例通过（两映射、Done/Error 简洁不发声、rows 仅两行） <!-- comet-task:7ac03056-de76-41eb-a3f0-201c7ec7acf9 -->
- [x] 6.2 前端与授权清单同步：简洁方案映射区仅渲染开始/处理两行并追加「完成与失败不发声」说明行，试听按钮播实际音源；LICENSE.md 的 ding/soft-alert 用途改回「可选」、tink/pop 标注为简洁方案兜底音 — 验证：`npm run build` 通过，界面核对两种形态 <!-- comet-task:911fad23-0035-4e73-9d23-4b17f12eb909 -->
- [x] 6.3 开始音可闻性修复（走查缺陷）：`start_recorder` 时序对齐 cometix-dictation（开麦/转正 → 播开始音 → 延迟压音量），预开麦路径转正时先还原音量再发声；压音量按 `sound::play_duration`（系统音已知时长 / WAV 时长 RIFF 块链解析 + 投递余量）延迟到提示音播完，且仅在会话进行中生效；两个方案的开始音均不再被「录音时调整音量」压掉 — 验证：用户实机走查确认开始音完整可闻；`cargo test` 全量通过（现有测试基建无法对时序做 RED；WAV 时长解析有 RIFF 块链与真实文件回归单测——最终集成审查曾发现固定偏移对真实文件返回 0ms，已修复，如实记录） <!-- comet-task:c52d5248-9af6-425b-9c93-eb8bde365ac1 -->

## Workflow follow-up

- Verify 阶段通过后由 `/comet-archive` 归档，同步 `docs/openspec/specs/audio-feedback/spec.md`
- 归档与交付方式（local/push/pr）在 Archive 阶段按用户选择执行
