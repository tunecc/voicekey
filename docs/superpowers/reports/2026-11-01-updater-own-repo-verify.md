# 验证报告：updater-own-repo

- 日期：2026-11-01
- verify_mode：full（规模评估：任务 6 > 3）
- 审查模式：standard（Verify 唯一一次最终集成审查：独立审查者全量 diff 核验 + 修复复验）
- 基线：base-ref 7e0f396（计划 frontmatter）；未提交改动均属本 change

## 完整性（Completeness）

| 项 | 结果 |
|---|---|
| tasks.md 任务完成 | 6/6 `[x]`（含用户动作 1.3：Secrets 已经 `gh secret set` 配置并经 `gh secret list` 确认） |
| delta spec 覆盖 | app-update-channel 2 条需求全部有实现与核验映射 |
| 3.2 场景对照 | endpoints/pubkey/URL 生成逻辑静态核验通过；端到端打 tag 验证列入 Workflow follow-up（需用户补配 macOS 证书 Secrets 后执行） |

## 正确性（Correctness）：规格场景 → 证据映射

### app-update-channel / 更新检查指向本仓库发布源

| 场景 | 证据 |
|---|---|
| 更新检查请求本仓库 latest.json | `tauri.conf.json` `endpoints=["https://github.com/tunecc/voicekey/releases/latest/download/latest.json"]`（审查逐字比对 specs）；`update.rs` 纯走 tauri-plugin-updater 读此配置，无第二处硬编码更新源 |
| 发现新版本/已是最新 | 运行时行为由 tauri-plugin-updater 按配置执行；端到端验收依赖打 tag（Workflow follow-up），静态链路已核验（release.yml 以 `$GITHUB_REPOSITORY` 生成 URL 与 latest.json） |

### app-update-channel / 更新包签名校验

| 场景 | 证据 |
|---|---|
| 自有密钥签名通过校验 | `pubkey` 与 `~/.tauri/voicekey.key.pub` 逐字节一致（审查实测对比）；密钥对本机生成（key ID `BCE2E93C2B1780AB` ≠ 上游 `C47134B02313979F`）；`tauri signer sign` 试签成功；Secret `TAURI_SIGNING_PRIVATE_KEY` 已配置（gh secret list 确认） |
| 签名不匹配拒绝安装 | tauri-plugin-updater 既有验签行为，配置侧无改动；拒绝路径不影响当前版本（上游行为保持） |

## 一致性（Coherence）

- D1（endpoints/pubkey 直写配置）、D2（密钥同源 + Secrets）、D3（README 单一入口）忠实落实；diff 仅含 `tauri.conf.json` 两行与 README 两处，无夹带改动。
- 私钥安全：审查以密钥材料探针扫描整个工作区（含未跟踪文件）零命中；私钥仅存 `~/.tauri/`（仓库外）与 GitHub 加密 Secrets。
- 无口令私钥取舍已在 design Risks 披露，README §6 如实记录生成命令（`--ci`）。

## 检查命令与结果

| 命令 | 结果 |
|---|---|
| `python3 -m json.tool app/src-tauri/tauri.conf.json` | 通过 |
| `cargo check -p voicekey` | 通过（3 个预存在警告未恶化；Runtime 证据 exit=0 已录制） |
| `npm run build`（app） | 通过 |
| `tauri signer sign` 试签 | 成功（密钥可用性） |
| `gh secret list --repo tunecc/voicekey` | `TAURI_SIGNING_PRIVATE_KEY` 存在 |

## 最终集成审查（Verify 唯一一次）

- 独立审查者全量核验（第 1 轮）：配置改动三方一致（specs/design/本机密钥）；发现 2 × IMPORTANT——① README §6 示例 tag `v1.7.2-fork.1` 不符合 MSI 预发布段规则会导致发版失败且与 §4 自相矛盾；② `MACOS_CERTIFICATE` Secrets 未配置属打 tag 前置（已在文档披露，属 follow-up 门禁）；1 × MINOR（失败后果表述偏轻）。
- 修复：①② 项已修（示例改 `v1.7.3` 并注明 MSI 版本规则；后果句改为「release job 依赖全部平台构建成功，任一平台失败都不会发布 Release 和 latest.json」）。README 属 verify 允许编辑的中性文档，证据不作废。
- 修复复验（第 2 轮）：**APPROVED**——两处修复确认有效（示例 tag 与 MSI 规则一致、失败后果句与 release.yml 结构相符），无新问题，diff 范围未变，报告工件确认不含私钥材料。

## 问题与处理

- CRITICAL：无
- IMPORTANT：2 项（① 已修复并复验；② 属用户运维前置——打 tag 前需补配 `MACOS_CERTIFICATE`/`MACOS_CERTIFICATE_PASSWORD`，README 已披露并列入 follow-up 门禁，不阻塞本 change 归档）
- WARNING：无
- SUGGESTION：无待处理项

## 结论

全部检查通过，无 CRITICAL/IMPORTANT 遗留。**PASS**
