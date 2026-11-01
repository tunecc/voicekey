# Tasks

## 1. 更新通道配置

- [x] 1.1 `app/src-tauri/tauri.conf.json`：updater `endpoints` 改为 `https://github.com/tunecc/voicekey/releases/latest/download/latest.json`。验证：JSON 解析通过（`python3 -m json.tool`），值与 specs 一致 <!-- comet-task:f756a6c2-a228-42cc-ba93-c91025a90864 -->
- [x] 1.2 `tauri.conf.json`：updater `pubkey` 替换为本机生成的 minisign 公钥（`~/.tauri/voicekey.key.pub` 内容，无口令密钥，已实测可签名）。验证：写入后 JSON 解析通过，公钥与私钥同源（`tauri signer sign` 试签 + 应用配置侧公钥同源生成） <!-- comet-task:2e83a875-14ce-4b3e-b526-7c9ee4458e39 -->
- [x] 1.3 前置确认：fork 仓库 Secrets 已配置 `TAURI_SIGNING_PRIVATE_KEY`（私钥内容；无口令密钥无需口令 Secret）。验证：`gh secret list --repo tunecc/voicekey` 或仓库 Settings 页面可见该名称 <!-- comet-task:a864a9f9-f687-405e-af48-ab61f03b318a -->

## 2. 文档

- [x] 2.1 `README.md` fork 差异章节新增「更新通道与签名密钥」：密钥对生成、Secrets 配置、pubkey 写入位置、打 tag 发版一次验证的步骤。验证：README 渲染检查（步骤完整可照做），与 design.md D3 一致 <!-- comet-task:bf872e1e-b277-4e44-ab57-696940f06e25 -->

## 3. 集成验证

- [x] 3.1 构建：`npm run build`（app）与 `cargo check -p voicekey` 通过，确认 updater 配置合法。验证：命令实际执行并记录结果 <!-- comet-task:0a484a12-0d67-4d7f-8449-f83a30c06eca -->
- [x] 3.2 发布链路验收（需打 tag，可放在发布时验证）：fork 打 tag 触发 Release → latest.json 出现在 tunecc/voicekey Releases 且 URL 指向本仓库 → 本 change 之后的版本可完成校验安装。验证：按 specs/app-update-channel 场景逐条记录结果；无法立即打 tag 时在 README 步骤中注明验证时点 <!-- comet-task:5e1ba350-0ea6-4b72-92c5-5a2abfc778d3 -->

## Workflow follow-up

- 交互式发布链路手动验证（打 tag 后）：按 specs/app-update-channel 场景逐条走查并记录。
- Verify 阶段完成唯一一次最终集成审查后，按归档流程确认归档与交付方式。
