# 验证报告：simple-sound-scheme

- 日期：2026-10-31
- 验证模式：full（任务 15 > 3，delta spec 能力 1）
- 独立审查：Build 阶段任务级审查（首轮 FAIL→修复→PASS；纯双音修订轮 FAIL→修复→PASS）+ Verify 阶段最终集成审查（FAIL I1→修复→定向复查 PASS），共 5 轮审查交互
- 走查修订：用户实机走查后发起两次范围修订（纯双音复刻、开始音可闻性），均经用户明确确认后实施

## Summary

| 维度 | 状态 |
| --- | --- |
| Completeness | 15/15 任务完成；4 个 MODIFIED 需求全部实现 |
| Correctness | 规格场景通过（用户实机走查确认 + 单测覆盖）；最终集成审查 PASS |
| Coherence | 设计决策 D1–D8 全部遵循；走查修订已同步 design.md/spec/tasks（revise-design/revise-plan 均已登记批准） |

## 检查项（full 验证）

| # | 检查项 | 结果 | 证据 |
| --- | --- | --- | --- |
| 1 | tasks.md 全部任务完成 | PASS | Runtime 核对：15/15 已勾选 |
| 2 | 实现符合 design.md 高层决策 | PASS | 独立审查逐点核对 D1（单字段门控）、D2（纯双音+系统音）、D3（常量表单一来源命令）、D4（方案单选条件渲染）、D8（NSSound 主线程播放+回退链） |
| 3 | 实现符合 Design Doc | PASS | 本 change design.md 即正式技术设计；走查修订经 revise-design 重新批准，文档无内部矛盾（Context/Non-Goals 已同步） |
| 4 | 能力规格场景全部通过 | PASS | 见下方场景覆盖明细 |
| 5 | proposal.md 目标已满足 | PASS | 简洁/自定义方案并存、macOS 与 cometix 同声、方案切换持久化、素材 CC0 入池均已交付 |
| 6 | delta spec 与 design doc 无矛盾 | PASS | 4 个 MODIFIED 需求与 D1–D8 一致；走查修订双侧同步（spec 引言、Non-Goals 均已收敛为纯双音语义） |
| 7 | 关联设计文档可定位 | PASS | docs/openspec/changes/simple-sound-scheme/design.md + docs/superpowers/plans/2026-10-31-simple-sound-scheme.md（任务 ID 映射完整） |

## 规格场景覆盖明细

**用户实机走查（2026-10-31，确认「OK了」）**：

- 简洁方案开始音（系统 Tink）完整可闻（6.3 时序修复后确认）
- 设置页音效方案单选、简洁/自定义切换、逐事件选择保留
- 试听按钮播放（简洁方案试听走系统音路径，「试听即所得」）
- soundScheme 持久化（跨越多次 dev 重启保持）

**自动化/审查证据**：

- 简洁门控（开始=tink、处理=pop、完成/失败不发声）、总开关两方案全静音、方案切换只读不改逐事件字段：`sound.rs` 单元测试
- WAV 时长解析：RIFF 块链合成用例（含 LIST 元数据块）+ 真实池内文件回归（tink≈55/pop≈72/ding≈300/switch≈124ms）——回归点为最终集成审查发现的固定偏移 0ms 缺陷（I1）
- serde camelCase 键名序列化守卫（首轮审查 C1 修复）
- Windows 编译：`play_system_quietly` 非 macOS 空桩；审查者以真实 x86_64-pc-windows-msvc 目标最小实证（修复前 E0425 / 修复后通过）
- 缺省字段兜底（无 soundScheme 旧配置 → custom）、非法 scheme 回退：`settings.rs` 单元测试
- 取消不发声、触发点不变：controller.rs 触发点未改动（独立审查核实），取消路径无发声代码
- 用户旧配置（无 soundScheme 字段）实机加载即默认自定义方案：走查期间实测（其真实 settings.json 即旧格式）

**已知接受的偏差（SUGGESTION 级，不阻塞）**：

- 设置页映射行显示兜底 WAV 中文名（轻叮/轻啵），macOS 实际发声为系统音 Tink/Pop——名称与音源轻微错位，审查评估可接受
- 简洁方案 macOS Pop 压音量点余量约 6ms（tokio sleep 只晚不早，实际影响可忽略；如走查发现尾音被削可将常量增至 ~85ms）
- tasks.md 1.1/3.1 历史文本仍描述四音/四行（任务文本不回改，归档核对即可）

## 审查问题闭环记录

| 轮次 | 发现 | 处置 |
| --- | --- | --- |
| Build 首轮 | C1：SimpleSchemeRow serde 键名 snake_case，前端读 undefined | 修复 rename_all + 键名守卫单测 → 复查 PASS |
| 纯双音修订轮 | C1：play_system_quietly 无 cfg 门控，Windows 编译失败 | 非 macOS 空桩 + 真实 Windows 目标最小实证 → 复查 PASS |
| Verify 集成审查 | I1：wav_duration_bytes 固定偏移对真实文件（含 LIST 块）返回 0ms | RIFF 块链解析 + 真实文件回归单测 → 定向复查 PASS |

## 构建与检查证据

- `cargo test`（workspace，Runtime 检查实跑）：27 passed（app）+ 21 passed + 3 ignored（core）
- `npm run build`（vite）：通过；`node --test src/lib/store.test.mjs`：3 passed
- macOS 实机：dev 实例用户走查通过（简洁方案系统音、试听、切换、持久化）
