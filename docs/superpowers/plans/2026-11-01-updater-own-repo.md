---
change: updater-own-repo
design-doc: docs/openspec/changes/updater-own-repo/design.md
base-ref: 7e0f39604492579f3bbd970ff674460cf19b8b8f
archived-with: 2026-11-01-updater-own-repo
---

<!-- comet-task-authority: docs/openspec/changes/updater-own-repo/tasks.md -->

# 实施计划：updater-own-repo

策略：autonomous + TDD + standard 审查。本 change 为配置与文档改动，无新增可单元测试的代码路径——TDD 以「验证式核验」落实（JSON 校验、试签、构建检查），不伪造 RED；这与 settings-none-options 中核验类任务的处理一致。设计依据：design.md D1–D3；验收：specs/app-update-channel。

## 1. 更新通道配置

<!-- comet-task-ref:f756a6c2-a228-42cc-ba93-c91025a90864 -->
**1.1 endpoints 指向本仓库**（依赖：无）
- 改动：`app/src-tauri/tauri.conf.json` `plugins.updater.endpoints` → `["https://github.com/tunecc/voicekey/releases/latest/download/latest.json"]`。
- 验收：`python3 -m json.tool` 解析通过；值与 specs 一致。

<!-- comet-task-ref:2e83a875-14ce-4b3e-b526-7c9ee4458e39 -->
**1.2 pubkey 替换为自有公钥**（依赖：1.1）
- 改动：`plugins.updater.pubkey` → `~/.tauri/voicekey.key.pub` 文件内容（本机生成，无口令，已实测 `tauri signer sign` 可用）。
- 验收：JSON 解析通过；公钥与私钥同源（同一 `signer generate` 输出，试签已验）。

<!-- comet-task-ref:a864a9f9-f687-405e-af48-ab61f03b318a -->
**1.3 Secrets 配置确认**（依赖：1.2；用户动作）
- 维护者把 `~/.tauri/voicekey.key` 内容配到 fork 仓库 Secrets：`gh secret set TAURI_SIGNING_PRIVATE_KEY --repo tunecc/voicekey < ~/.tauri/voicekey.key`（无口令密钥无需口令 Secret）。
- 验收：`gh secret list --repo tunecc/voicekey` 可见该名称；由维护者执行并确认。

## 2. 文档

<!-- comet-task-ref:bf872e1e-b277-4e44-ab57-696940f06e25 -->
**2.1 README「更新通道与签名密钥」**（依赖：1.1/1.2）
- 改动：README fork 差异章节新增小节——密钥生成命令、Secrets 配置、pubkey 写入位置、打 tag 验证发版的步骤与「验证时点」说明。
- 验收：步骤完整可照做，与 design.md D3 一致。

## 3. 集成验证

<!-- comet-task-ref:0a484a12-0d67-4d7f-8449-f83a30c06eca -->
**3.1 构建检查**（依赖：1–2）
- `cargo check -p voicekey`、`npm --prefix app run build`（或 app 下 `npm run build`）。验收：退出码 0，`comet check run` 录制证据。

<!-- comet-task-ref:5e1ba350-0ea6-4b72-92c5-5a2abfc778d3 -->
**3.2 发布链路验收对照**（依赖：3.1）
- 以自动化/静态核验对照 specs 场景：endpoints 值、pubkey 与私钥同源、release.yml 按 `$GITHUB_REPOSITORY` 生成 URL；打 tag 后的端到端走查列入 Workflow follow-up。

## 自检

- 任务与 tasks.md 一一对应（6 项），无第二套 checkbox。
- 无范围外任务；TDD 以验证式核验如实记录，不伪造 RED。
- 回退：还原 tauri.conf.json 与 README 即可；配置无迁移语义。
