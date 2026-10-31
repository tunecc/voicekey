---
change: simple-sound-scheme
design-doc: docs/openspec/changes/simple-sound-scheme/design.md
base-ref: a605787ad0de675a0110dfdd0c3db628afd90a42
archived-with: 2026-10-31-simple-sound-scheme
---

<!-- comet-task-authority: docs/openspec/changes/simple-sound-scheme/tasks.md -->

# Plan: simple-sound-scheme

新增「简洁」音效方案（复刻 cometix-dictation 极简风格 + CC0 协调音效补齐），与现有「自定义多音效」并存。技术方案见 Design Doc（D1–D4），行为要求见 `docs/openspec/changes/simple-sound-scheme/specs/audio-feedback/spec.md`。执行配置：autonomous + tdd + standard 审查；实现任务先 RED 后 GREEN，命令与结果记入任务验收。

## 任务依赖与顺序

1.x（素材）→ 1.3（音池登记）→ 2.x（Rust 门控，依赖 1.3 的 id）→ 3.1（命令，依赖 2.x 常量表）→ 4.x（前端，依赖 3.1）→ 5.x（集成验收）。

## Phase 1 音效素材检索与入库

<!-- comet-task-ref:ec7930d8-6234-4370-a4d7-66ee14299f80 -->
**1.1 检索与挑选 4 个 CC0 音效**
- 范围：上网检索候选（优先 Kenney Interface Sounds 镜像仓库 Calinou/kenney-interface-sounds 的完整清单，其次 Pixabay/Mixkit/freesound CC0 过滤），按音色特征挑选：开始=tink 类轻巧短敲音、处理=pop 类轻弹音、完成=轻快确认音、失败=低调柔和短音；每个 ≤0.3s、试听后定稿；记录来源页/原始文件名/授权。
- 约束：只取 CC0/公有领域；不得使用 Apple 系统音；与现有 6 音色风格协调。
- 验收：4 个原始音频文件入库临时目录，来源清单可追溯（写入 LICENSE.md 草稿内容）。

<!-- comet-task-ref:03fa2e7e-5fd3-48d6-a7a3-b296bbf0cdf5 -->
**1.2 转码入库**
- 范围：ffmpeg 裁首尾静音（-45dB 阈值）→ mono → 44.1kHz/16-bit PCM WAV；命名 `tink.wav`/`pop.wav`/`ding.wav`/`soft-alert.wav`（id 在实现时如与素材特征不符可调整，保持语义）；放入 `app/src-tauri/resources/sounds/`；LICENSE.md 增补表格行。
- 验收：`file` 输出为 44100 Hz mono 16-bit WAV；LICENSE.md 含 4 个新条目。

<!-- comet-task-ref:120a36dc-fc9a-4525-99c4-6015b6a46af3 -->
**1.3 SOUNDS 池登记**
- 范围：`sound.rs` `SOUNDS` 常量追加 4 个 `(id, 中文名)`（轻叮/轻弹/清亮/低鸣 等中文名定稿）。
- 验收（TDD）：先确认现有测试 `sound_pool_is_not_empty_and_ids_match_wav_files` 对新 id 失败（RED），登记后通过（GREEN）。

## Phase 2 Rust 侧方案门控

<!-- comet-task-ref:35e24cc4-c508-4ece-8632-c3748bb2b0a1 -->
**2.1 sound_scheme 字段**
- 范围：`settings.rs` 新增 `sound_scheme: String`（默认 `"custom"`、`#[serde(default)]`、sanitize 非法值回退 `"custom"`）；`app/src/lib/api.ts` Settings 类型镜像。
- 验收（TDD）：新增单测——缺字段 serde 兜底为 custom；`sanitize` 把 `"weird"` 回退 custom；合法值保留。RED：测试先行失败；GREEN：实现后通过。

<!-- comet-task-ref:f0265e22-2f91-4972-b391-2605efe0eec5 -->
**2.2 固定映射与门控分支**
- 范围：`sound.rs` 新增 `SIMPLE_SCHEME` 常量映射（SoundEvent → 音效 id）与 `gated_id` 的 `simple` 分支；`custom` 路径不变；总开关优先级不变。
- 验收（TDD）：新增单测——simple 下四事件返回映射 id；custom 下沿用逐事件字段；总开关关闭时两者均 None；映射 id 均在池内。RED→GREEN。

<!-- comet-task-ref:1558469e-773d-4b92-80b0-bcf9d2314113 -->
**2.3 方案切换不改逐事件字段**
- 范围：核对 `gated_id(simple)` 与设置保存路径均为只读引用；单测断言 simple 门控不修改四个逐事件字段。
- 验收（TDD）：单测先行 RED→GREEN。

## Phase 3 Tauri 命令

<!-- comet-task-ref:0a777fbe-8264-446b-ab92-c8b76e811e42 -->
**3.1 simple_scheme() 命令**
- 范围：`main.rs` 新增并注册 `simple_scheme()`，返回 `[{event, eventLabel, soundId, soundLabel}]`，数据由 `SIMPLE_SCHEME` + `SOUNDS` 生成。
- 验收（TDD）：Rust 单测断言映射完整性（四事件、id 在池内、标签非空）；命令注册后 `cargo build` 通过。

## Phase 4 前端设置页

<!-- comet-task-ref:5ded6c17-7955-49dc-835b-1291db082d2b -->
**4.1 api.ts 封装**
- 范围：`soundScheme` 字段镜像（含默认值）、`simpleScheme()` 调用封装与返回类型。
- 验收：`npm run build`（或现有前端构建命令）通过。

<!-- comet-task-ref:af744057-e00b-4c16-913d-bd212b744ced -->
**4.2 方案单选行**
- 范围：`Audio.svelte` 提示音组顶部加「音效方案」分段单选（简洁/自定义），变更 `save({ soundScheme })` 即时持久化；沿用 `.group/.row` 结构。
- 验收：界面切换后 `settings.json` 的 `soundScheme` 更新、重启保持（手动核对，记录结果）。

<!-- comet-task-ref:ca8a65a3-fca5-46dd-b106-710e75105faa -->
**4.3 条件渲染与试听**
- 范围：`soundScheme === "simple"` 时隐藏四行逐事件 `select`，渲染只读映射行（事件名+音效名+试听按钮，试听调 `previewSound(映射id)`）；`"custom"` 时恢复逐事件行，选项值来自既有字段（不清空）。
- 验收：两种形态切换核对 + 试听播对应音效（手动核对，记录结果）。

## Phase 5 集成验收

<!-- comet-task-ref:d703345d-8558-41ad-a2ea-0b1128ff3c41 -->
**5.1 端到端场景验收**
- 范围：按 delta spec 场景逐条验收（方案切换即时生效、固定映射发声、自定义逐事件、取消静音、总开关全静音、试听、持久化、缺省兜底）。
- 验收：手动运行应用逐场景核对并记录。

<!-- comet-task-ref:625b9c47-7031-418c-b55e-32ac33bf62dd -->
**5.2 旧配置回归**
- 范围：用不含 `soundScheme` 的旧 `settings.json` 启动，确认默认 custom、逐事件行为与升级前一致。
- 验收：手动核对并记录。

## Phase 6 纯双音复刻（走查后范围修订，用户已确认 2026-10-31）

<!-- comet-task-ref:7ac03056-de76-41eb-a3f0-201c7ec7acf9 -->
**6.1 Rust 纯双音与 macOS 系统音**
- 范围：`SIMPLE_SCHEME` 收敛为开始/处理两映射（简洁方案下完成/失败不发声）；新增 `simple_scheme_system_name`（macOS→"Tink"/"Pop"）；`play`/`play_id` 增加系统音路径（objc2-app-kit NSSound 主线程播放，`run_on_main_thread`；系统音不可用或非 macOS 回退池内兜底音）；Cargo.toml 启用 NSSound/NSString 特性（依赖已在树中）。
- 验收（TDD）：新增/修订单测——映射仅两项且在池内、Done/Error 简洁不发声、系统音名按平台解析、rows 仅两行；`cargo test` 全量通过。

<!-- comet-task-ref:911fad23-0035-4e73-9d23-4b17f12eb909 -->
**6.2 前端与授权清单同步**
- 范围：`Audio.svelte` 简洁方案映射区仅渲染 schemeRows（两行）+「识别完成 / 识别失败：不发声」说明行；`LICENSE.md` 用途标注更新（ding/soft-alert→可选，tink/pop→简洁方案兜底音）。
- 验收：`npm run build` + `node --test` 通过；界面两种形态核对（并入 5.1 用户走查）。

<!-- comet-task-ref:c52d5248-9af6-425b-9c93-eb8bde365ac1 -->
**6.3 开始音可闻性修复（走查缺陷）**
- 范围：`start_recorder` 时序对齐 cometix-dictation（开麦/转正 → 播开始音 → 按 `sound::play_duration` 延迟压音量；预开麦转正先还原音量）；`Msg::LowerVolume` 延迟投递且仅在会话进行中生效。
- 验收：用户实机走查确认开始音完整可闻（已确认）；`cargo test` 全量通过（时序无 RED 基建，WAV 头解析有单测）。

## 风险与回退

- 音色听感不符：只替换 `resources/sounds/*.wav` 与 LICENSE 行，不动代码（id 稳定）。
- 实现缺陷回退：整个 change 可基于 base-ref `a605787` 回退；无数据迁移。

## 提交策略

每个 Phase 完成并通过对应检查后提交一次（含 delta spec / handoff 变更说明）；最终提交在 guard build --apply 通过之后完成剩余勾选。
