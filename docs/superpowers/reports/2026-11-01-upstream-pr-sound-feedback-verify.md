# 验证报告 — upstream-pr-sound-feedback（轻量验证）

日期：2026-11-01 ｜ 工作流：classic/tweak ｜ 验证模式：light ｜ 审查模式：off（已做独立集成审查补偿）

## 交付物

- PR 分支 `feat/session-sound-feedback`（基线 upstream/main@25b3f50 v1.9.0），位于独立 worktree `.worktrees/pr-sound-feedback`，提交 `4509ac6`（单个 feat 提交，20 文件，+695/−5 行）
- PR 标题/描述草稿：`docs/openspec/changes/upstream-pr-sound-feedback/pr-draft.md`
- 本仓库（fork）代码零改动；仅 docs/openspec 流程产物

## 轻量验证 7 项

| # | 检查项 | 结果 | 证据 |
| --- | --- | --- | --- |
| 1 | tasks.md 全部勾选 | PASS | 11/11 `[x]`（1.1–4.1） |
| 2 | 改动文件与 tasks 一致 | PASS | `git diff upstream/main..HEAD --stat` = 关键文件清单（8 改 + sound.rs + resources/sounds/），排除清单零混入；逐 hunk 复核（settings.rs 纯新增、controller.rs 仅提示音 hunks、main.rs 保留 jev_test、Cargo.lock 仅 rodio/objc2 条目、tauri.conf.json 仅 bundle.resources） |
| 3 | 编译通过 | PASS | Runtime 记录：`cargo test --workspace`（nightly，含全量编译）退出码 0；`cargo check -p voicekey` 0 警告 |
| 4 | 相关测试通过 | PASS | voicekey 24（含 13 个提示音相关 + 1 个新增时长估计测试）、voicekey-core 51、voicekey-platform 14、voicekey-wtlocal 1；0 失败。vite build 退出码 0 |
| 5 | 无明显安全问题 | PASS | 无硬编码密钥；无新增 unsafe（较 fork 版净减少 2 处 unsafe 包装）；WAV 解析有长度钳制与 byte_rate 防护 |
| 6 | 最终集成审查 | PASS | review_mode=off 跳过自动审查已记录；补偿性独立审查者（子代理）完成全 diff 审查：无 CRITICAL；1 个 IMPORTANT 当场修复（见下）；正确性/泄漏/边界/前端四项均通过 |
| 7 | 核心场景 | PASS（代码层） | 触发点（开始/处理/失败）与门控、默认值（Tink/Pop/静默）、sanitize 回退、「无」保留、会话代数门闩、450ms 恢复窗口均有单测或逐 hunk 对照 fork 一致性背书；实机听感验证不在本环境可执行范围，随 PR 由上游与用户实测（同 remove-custom-sound-scheme 2.2 的情况） |

## 审查发现与处置

- **IMPORTANT（已修复）**：`play_duration` 仅覆盖 Tink/Pop 固定时长，无兜底 WAV 的系统音作开始音且开启压低音量时，ducking 立即触发、开始音被切断（fork 同源缺陷）。修复：新增 `estimate_duration`——无实测数据的系统音按 250ms 保守发声体估计（实测系统音全长含混响尾音，Tink 全长 564ms 而可听主体约 60ms，不能按全长延迟），并新增测试 `duration_estimate_covers_all_system_sounds` 保证全部系统音估计 > 0。修复以 `--amend` 并入单一 feat 提交（4509ac6）。
- **WARNING（接受，保持 fork 一致）**：①「识别完成」行有选择器但结果就绪不接线发声——现行 spec 明确设计（完成信号由处理音承担），PR 描述已向上游说明；②捆绑 10 个 WAV 中 5 个运行时零引用（fork 音效池遗留，约 72KB）——为保持与 fork 交付一致与最小 diff 予以保留。
- **SUGGESTION（记录，不在本 PR 处理）**：音频线程首启失败永久缓存 None；文件打开错误会重建健康输出流；`save({[row.key]})` 在严格 tsc 下的宽松类型；界面音效名显示英文 id。

## 偏差与既有限制（如实记录）

1. **`cargo fmt --check` 门禁不可用**：本地 rustfmt（stable 1.97 / 裸 rustfmt）对纯净 upstream/main 报 338 处 diff（import 顺序、单行拆分等），上游不执行 CI fmt，属仓库既有风格漂移；为不污染 PR diff 未执行格式化。已单独核验新增代码与仓库现行风格一致（移植自 fork 同源风格）。
2. **stable 工具链无法编译上游基线**：rustc 1.95/1.97 均在 `crates/wtlocal/src/kernels.rs` 报 E0658（`vdotq_s32` 仍在 unstable `stdarch_neon_dotprod` 之后，tracking #117224；上游 CI 通过时用的是更旧的 stable）。本 PR 不触碰 crates/，验证在 nightly 1.101 上完成；PR 描述已附注此既有问题并提议可另开 PR 修复。
3. **任务文本笔误**：1.2 写「11 个 WAV」，实际为 10 个 WAV + LICENSE.md（fork 实际资产数）；意图无歧义，已按实际资产交付（批准后任务文本不可改，此处如实记录）。
4. **实机听感**：开/关提示音、试听、压低音量时序的最终实机验证未在本环境执行（无音频回路自动化手段）；代码路径均有单测或 fork 同源一致性背书。

## Runtime 检查记录

- build：`rustup run nightly cargo test --workspace`（cwd=项目根）退出码 0，经 `comet guard build --apply` 内联执行记录
- verify：同命令按「argv/cwd/输入/环境一致可跨阶段复用」复用；vite build 本会话实际执行退出码 0
