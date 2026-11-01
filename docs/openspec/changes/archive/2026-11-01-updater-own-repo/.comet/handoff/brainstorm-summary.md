# Brainstorm Summary

- Change: updater-own-repo
- Date: 2026-11-01

## 确认的技术方案

沿用上次已确认方案：`tauri.conf.json` 的 updater `endpoints` 指向 `https://github.com/tunecc/voicekey/releases/latest/download/latest.json`，`pubkey` 换成本机生成的 minisign 公钥；README 补「更新通道与签名密钥」章节（生成/Secrets/验证步骤）。不改 release.yml、离线模型库 URL、应用标识。本次唯一更新：密钥对已在本机生成（`~/.tauri/voicekey.key`，无口令，`tauri signer generate --ci`，实测可签名），公钥写入配置；Secrets 只需 `TAURI_SIGNING_PRIVATE_KEY` 一项（无口令密钥，口令 Secret 留空合法），由维护者经 `gh secret set` 或网页配置，口令不经 Agent。

确认依据：用户在 AskUserQuestion 明确选择「确认本设计」。

## 关键取舍与风险

- 无口令私钥：拿到密钥文件即可签名（个人 fork 常见取舍）；密钥仅存本机与 GitHub 加密 Secrets，不入库。
- 旧 fork 安装版本带上游公钥，收不到 fork 更新——本 change 发布的版本起恢复。
- 发布链路验收依赖打 tag（README 注明验证时点）。

## 测试策略

配置 JSON 解析校验、`cargo check -p voicekey`、`npm run build`（app）；`tauri signer sign` 试签验证密钥可用；发布链路（latest.json、验签安装）需打 tag 后按 specs 场景走查，记入 Workflow follow-up。

## Spec Patch

无——delta spec（app-update-channel）已覆盖全部验收场景。
