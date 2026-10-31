# Brainstorm Summary

- Change: simple-sound-scheme
- Date: 2026-10-31

## 走查后修订（2026-10-31，用户确认纯双音复刻）

用户实机走查后反馈 CC0 近似音与 cometix-dictation 实际听感不同，要求纯双音直接套用：

- 简洁方案改为**纯双音**：开始=**Tink**、处理=**Pop**；完成与失败**不发声**（cometix 本就没有）
- **macOS 直接播放系统音 Tink/Pop**（NSSound(named:)，与 cometix 同 API 同声音；运行时引用系统目录，不打包不分发，规避版权）；经 `AppHandle::run_on_main_thread` 主线程调用，与 cometix 的主线程策略一致
- **Windows 回退内置 CC0 近似音** tink.wav/pop.wav（此前挑选的 tick_004/drop_003）；系统音不可用时 macOS 同样回退
- `ding`/`soft-alert` 保留在音效池（自定义方案可选），不再被简洁方案引用
- 技术路径：objc2-app-kit（已在依赖树 tauri→muda）启用 NSSound 特性；试听即所得——简洁方案试听走同一发声源解析
- 规格同步：会话事件提示音/提示音设置/音效试听/内置音效与跨平台一致 四个需求修订；"两平台音效集合一致"收敛为"自定义方案集合一致，简洁方案语义一致"

## 确认的技术方案（用户已于 2026-10-31 确认，proposalHash a094cb1b）

沿用现有音效架构（rodio 播放 + `sound.rs` 单点门控 + 扁平设置字段），增量实现两方案并存：

1. **门控**：`Settings` 新增 `sound_scheme: String`（`simple|custom`，默认 `custom`，serde default + sanitize 回退）。`gated_id()` 加分支：`simple` 查固定映射常量表，`custom` 沿用逐事件字段；总开关逻辑不变。切换方案只写 scheme 字段，不清逐事件字段（切回自定义恢复原选择）。
2. **映射与素材**：简洁方案固定映射——开始=tink 类轻巧短音、处理=pop 类轻弹音、完成=轻快确认音、失败=低调柔和短音；4 个 id 互不相同、注册进 `SOUNDS` 池。素材上网检索：优先 Kenney Interface Sounds 包内其余音色（授权已验证、风格统一），不足补其他 CC0 来源；≤0.3s、柔和不突兀、复刻 Tink/Pop 音色特征；ffmpeg 转码规范与 LICENSE.md 登记沿用现状。
3. **命令**：新增 `simple_scheme()` Tauri 命令，由 Rust 常量表生成映射数据供前端只读展示；`preview_sound(id)` 复用（试听即所得）。
4. **前端**：`Audio.svelte` 提示音组顶部加「音效方案」单选（`save({soundScheme})` 即时持久化）；simple 下隐藏逐事件下拉、渲染只读映射行（事件名+音效名+试听）；custom 恢复现有逐事件行。

## 关键取舍与风险

- 已否决：预设按钮改写逐事件字段（非真实并存、丢失用户选择）；macOS NSSound 系统音（不可再分发、Windows 无等价）；纯双音（完成/失败无反馈，用户要求补齐）。
- 风险：挑选音色与 Tink/Pop 听感有差距 → 同包内选变体逐个试听，换 WAV 不动代码；旧配置兼容 → serde default + sanitize 回退 custom；素材授权 → 只取 CC0 并逐文件登记。

## 测试策略

- Rust 单测：音池 id 与 WAV 文件对应、simple/custom 门控、sanitize 兜底、缺字段默认、方案切换不改逐事件字段。
- 前端：构建/lint + 界面形态切换与试听手动核对。
- 集成（Verify）：按 delta spec 15 场景逐条验收 + 旧配置回归。

## Spec Patch

无。Open 阶段 delta spec 已覆盖方案切换、固定映射、持久化、兜底与试听；brainstorming 未发现需补充的验收场景。
