---
comet_change: updater-own-repo
role: technical-design
canonical_spec: openspec
archived-with: 2026-11-01-updater-own-repo
status: final
---

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
