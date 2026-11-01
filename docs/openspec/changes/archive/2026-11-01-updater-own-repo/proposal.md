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
