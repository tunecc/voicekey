# Comet Design Handoff

- Change: updater-own-repo
- Phase: design
- Mode: compact
- Context hash: 90cfc79c98ca1b4318b40389155f9d47e53c388bb35011b86a31c55403d5ce56

Generated-by: comet-handoff.sh
Task hash policy: task-content-v1. Read tasks.md for live completion; excerpts are design-time context.

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## docs/openspec/changes/updater-own-repo/proposal.md

- Source: docs/openspec/changes/updater-own-repo/proposal.md
- Lines: 1-26
- SHA256: 59b89ada64910e1864220b0fe0167ebe76987cbfd6d5e05ed109f0bf037c2725

```md
# Proposal

## Why

本仓库是 J3n5en/voicekey 的个人 fork，应用内「检查更新」仍指向上游仓库的 `latest.json`，且校验公钥是上游的 minisign 公钥——fork 发自己的版本时用户拿不到更新，也无法用自己的密钥签名分发。

## What Changes

- `app/src-tauri/tauri.conf.json` 的 updater `endpoints` 改为本仓库 `https://github.com/tunecc/voicekey/releases/latest/download/latest.json`。
- updater `pubkey` 替换为 fork 自己的 minisign 公钥（已在本机生成密钥对 `~/.tauri/voicekey.key`，无口令，私钥不入库）。
- 维护者将私钥内容配置到本仓库 GitHub Secrets（`TAURI_SIGNING_PRIVATE_KEY`；无口令密钥无需配置口令 Secret）。
- 文档补充：密钥对生成、Secrets 配置、公钥写入位置的步骤；说明发版后应用内更新即指向本仓库。
- 发布链路（`.github/workflows/release.yml`）不改：latest.json 的下载地址已按 `$GITHUB_REPOSITORY` 生成，在 fork 上运行即指向 tunecc/voicekey。

## Capabilities

### New Capabilities

- `app-update-channel`：应用内更新检查的发布源与签名校验约定——更新检查必须请求本仓库的 latest.json，安装包必须通过应用内置公钥（fork 自有密钥）校验。

## Impact

- `app/src-tauri/tauri.conf.json`（endpoints、pubkey）。
- `README.md`（fork 差异说明中补充更新通道与密钥配置步骤）。
- 验证依赖发布链路：fork 打 tag 触发 Release 后，latest.json 位于 tunecc/voicekey Releases，应用内「检查更新」可发现并可安装。
- 非目标：离线模型库下载 URL（`offline.rs` / `Offline.swift`）不迁移；不改应用标识 `do.j3.voicekey`；不改 release.yml 构建与签名流程本身。

```

## docs/openspec/changes/updater-own-repo/design.md

- Source: docs/openspec/changes/updater-own-repo/design.md
- Lines: 1-48
- SHA256: a122ef6840e2c518f8c3759fb1b7d1b430d8b367cf95335f9058b2d20f654abd

```md
# Design

## Context

`app/src-tauri/tauri.conf.json` 的 `plugins.updater` 含 `endpoints`（现指向上游 `J3n5en/voicekey` 的 latest.json）与 `pubkey`（上游 minisign 公钥）。`.github/workflows/release.yml` 用 Secret `TAURI_SIGNING_PRIVATE_KEY(-PASSWORD)` 签名并在 `dist/latest.json` 里以 `$GITHUB_REPOSITORY` 生成下载地址——workflow 在 fork 上运行时产物 URL 已自动指向 tunecc/voicekey，因此发布链路无需改动，阻塞点只在应用侧的端点与公钥。`update.rs` 只调用 `tauri-plugin-updater`，端点与公钥全部来自配置文件。

## Goals / Non-Goals

**Goals:**

- 应用内更新只认本仓库发布源与本仓库密钥。
- 把「密钥生成 → Secrets 配置 → 公钥写入」固化为仓库文档，fork 维护者可独立完成发版。

**Non-Goals:**

- 不迁移离线模型库 URL（`offline.rs` / `Offline.swift`，大文件托管另立 change）。
- 不改应用标识 `do.j3.voicekey`（改动会使已安装用户丢失辅助功能等授权）。
- 不改 release.yml 的构建、签名、latest.json 生成逻辑。

## Decisions

### D1：endpoints 与 pubkey 直接写入 tauri.conf.json

- `endpoints: ["https://github.com/tunecc/voicekey/releases/latest/download/latest.json"]`。
- `pubkey` 替换为 fork 自有公钥。密钥对已在本机生成：`~/.tauri/voicekey.key`（私钥，base64 env 格式，无口令）与 `~/.tauri/voicekey.key.pub`（公钥），生成命令 `npx tauri signer generate -w ~/.tauri/voicekey.key --ci -f`，已用 `tauri signer sign` 实测无口令签名可用。私钥永不入库；公钥写入配置。
- 备选：运行时环境变量覆盖端点（TAURI_UPDATER_...）——否决：多一层配置面，fork 场景固定指向本仓库即可。

### D2：公钥与私钥的配对关系

应用内置公钥必须与 fork Secrets 中的私钥同源生成，否则「检查更新」能发现版本但安装校验失败。私钥以 `gh secret set TAURI_SIGNING_PRIVATE_KEY --repo tunecc/voicekey < ~/.tauri/voicekey.key`（或网页上传）配置到仓库 Secrets；无口令密钥无需配置 `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`（workflow 中该 env 为空即合法）。文档步骤中显式包含配对自检：`tauri signer sign` 试签 + 应用内更新走通。

### D3：文档位置

README 的 fork 差异章节补一节「更新通道与签名密钥」：生成密钥、配置 Secrets、写入 pubkey、打 tag 验证一次发版。不新增独立文档文件，保持 README 作为 fork 差异唯一入口。

## Risks / Trade-offs

- [无口令私钥：拿到密钥文件即可签名] → 个人 fork 的常见取舍；密钥只存在维护者本机与 GitHub 加密 Secrets，不入库。若日后需要更强保护，可重新生成带口令密钥并同步换 pubkey。
- [迁移期间已安装的旧 fork 版本仍带上游公钥] → 旧版本本来就收不到 fork 的更新（端点已指本仓库签名必败），本 change 发布的版本起恢复正常；不存在半校验状态。
- [GitHub Release 的 latest.json 依赖 tag 流程] → 属既有链路，docs 任务中包含一次发布验证步骤说明。

## Migration Plan

单配置文件改动 + README；无数据迁移。回滚 = 还原 tauri.conf.json 与 README。

## Open Questions

无。

```

## docs/openspec/changes/updater-own-repo/tasks.md

- Source: docs/openspec/changes/updater-own-repo/tasks.md
- Lines: 1-21
- SHA256: e953a407a43f0eb00a3da46fde5f947cb6b83a93e9e6b14c480c567fe8dd036f

```md
# Tasks

## 1. 更新通道配置

- [ ] 1.1 `app/src-tauri/tauri.conf.json`：updater `endpoints` 改为 `https://github.com/tunecc/voicekey/releases/latest/download/latest.json`。验证：JSON 解析通过（`python3 -m json.tool`），值与 specs 一致 <!-- comet-task:f756a6c2-a228-42cc-ba93-c91025a90864 -->
- [ ] 1.2 `tauri.conf.json`：updater `pubkey` 替换为本机生成的 minisign 公钥（`~/.tauri/voicekey.key.pub` 内容，无口令密钥，已实测可签名）。验证：写入后 JSON 解析通过，公钥与私钥同源（`tauri signer sign` 试签 + 应用配置侧公钥同源生成） <!-- comet-task:2e83a875-14ce-4b3e-b526-7c9ee4458e39 -->
- [ ] 1.3 前置确认：fork 仓库 Secrets 已配置 `TAURI_SIGNING_PRIVATE_KEY`（私钥内容；无口令密钥无需口令 Secret）。验证：`gh secret list --repo tunecc/voicekey` 或仓库 Settings 页面可见该名称 <!-- comet-task:a864a9f9-f687-405e-af48-ab61f03b318a -->

## 2. 文档

- [ ] 2.1 `README.md` fork 差异章节新增「更新通道与签名密钥」：密钥对生成、Secrets 配置、pubkey 写入位置、打 tag 发版一次验证的步骤。验证：README 渲染检查（步骤完整可照做），与 design.md D3 一致 <!-- comet-task:bf872e1e-b277-4e44-ab57-696940f06e25 -->

## 3. 集成验证

- [ ] 3.1 构建：`npm run build`（app）与 `cargo check -p voicekey` 通过，确认 updater 配置合法。验证：命令实际执行并记录结果 <!-- comet-task:0a484a12-0d67-4d7f-8449-f83a30c06eca -->
- [ ] 3.2 发布链路验收（需打 tag，可放在发布时验证）：fork 打 tag 触发 Release → latest.json 出现在 tunecc/voicekey Releases 且 URL 指向本仓库 → 本 change 之后的版本可完成校验安装。验证：按 specs/app-update-channel 场景逐条记录结果；无法立即打 tag 时在 README 步骤中注明验证时点 <!-- comet-task:5e1ba350-0ea6-4b72-92c5-5a2abfc778d3 -->

## Workflow follow-up

- 交互式发布链路手动验证（打 tag 后）：按 specs/app-update-channel 场景逐条走查并记录。
- Verify 阶段完成唯一一次最终集成审查后，按归档流程确认归档与交付方式。

```

## docs/openspec/changes/updater-own-repo/.openspec.yaml

- Source: docs/openspec/changes/updater-own-repo/.openspec.yaml
- Lines: 1-2
- SHA256: 108f7b3dae33488206672f31b510275110c0194c93062eb1970e19ca60c08722

```md
schema: spec-driven
created: 2026-11-01

```

## docs/openspec/changes/updater-own-repo/specs/app-update-channel/spec.md

- Source: docs/openspec/changes/updater-own-repo/specs/app-update-channel/spec.md
- Lines: 1-35
- SHA256: 072e0039419eec5b6cbe9ce96a0d02e3a76998e64b5f2205214e110172a10fcf

```md
# Spec Delta

## Purpose

应用内更新检查的发布源与签名校验约定：更新检查指向本仓库（tunecc/voicekey）的发布，安装包用本仓库自己的 minisign 密钥签名校验，使 fork 可以独立于上游分发可验证的更新。

## ADDED Requirements

### Requirement: 更新检查指向本仓库发布源

应用内更新检查（启动后自动定时检查与设置页手动「检查更新」）必须请求本仓库 Releases 的 `latest.json`；发现新版本时展示新版本号并提供「更新并重启」，已是最新时如实提示。检查失败时按现有静默/报错策略处理，不得回落到上游仓库的发布源。

#### Scenario: 发现本仓库发布的新版本

- **WHEN** tunecc/voicekey 发布了高于当前版本的新版本（含 latest.json）
- **THEN** 应用内「检查更新」显示新版本可用，点击「更新并重启」可完成下载安装并重启到新版本

#### Scenario: 已是最新版本

- **WHEN** 本仓库发布源中没有高于当前版本的版本
- **THEN** 「检查更新」提示已是最新版本

### Requirement: 更新包签名校验

应用必须使用应用内置的 minisign 公钥校验更新包签名，该公钥必须与本仓库发布时使用的签名私钥（GitHub Secret `TAURI_SIGNING_PRIVATE_KEY`）配对；签名校验失败的更新必须拒绝安装，且不得影响当前已安装版本的正常使用。

#### Scenario: 自有密钥签名的发布通过校验

- **WHEN** 本仓库 Release 的更新包由配对私钥签名
- **THEN** 应用内更新通过签名校验并正常安装

#### Scenario: 签名不匹配时拒绝安装

- **WHEN** 更新包签名与应用内置公钥不匹配（例如上游签名的发布）
- **THEN** 更新被拒绝安装并提示失败，当前版本继续正常使用

```
