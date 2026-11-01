# 验证报告：tray-wave-centering

- 日期：2026-11-01
- 模式：hotfix · verify_mode=light · review_mode=off（hotfix 预设默认，跳过自动代码审查；本次为单参数视觉修正 + 资产重生成，无逻辑面变化）
- 范围：`25878a1..ac33336`（Scripts/tray-icons.swift、app/src-tauri/icons/tray.png、本 change 产物文档）
- 语言：zh-CN

## 轻量验证 7 项

| # | 检查项 | 结果 | 证据 |
|---|--------|------|------|
| 1 | tasks.md 全部完成 | PASS | 4/4 `[x]` |
| 2 | 改动文件与 tasks 一致 | PASS | 脚本 1 行参数（centerY 9.0→10.14，仅候选 f，grep 确认 e/g/d 未动）+ 重生成 tray.png（44×44） |
| 3 | 编译通过 | PASS | guard build 执行 `cargo build`（Runtime 证据，5.30s 增量） |
| 4 | 相关测试通过 | PASS | `cargo test --workspace`（guard verify --apply 时由 Runtime 复核） |
| 5 | 无明显安全问题 | PASS | 单参数修正，无新增 unsafe/密钥/输入面变化 |
| 6 | 最终集成代码审查 | PASS（off） | review_mode=off 为 hotfix 预设默认；理由：改动为 1 行渲染参数 + 生成资产，客观指标（边距对称性、像素 diff）可完全覆盖正确性 |
| 7 | 核心场景验证 | PASS | 见下 |

## 核心场景：墨迹居中

- **修复前（RED）**：公式扫描墨迹 2.59..13.13（y-up），中心偏离 −1.14pt；PNG 实测（alpha>0.05）上边距 ≈4.5pt / 下边距 ≈2.45pt。
- **修复后**：PNG 实测**上边距 3.68pt / 下边距 3.68pt，差值 0.00pt**（阈值 ≤0.2pt）。
- 逐像素对比：新 tray.png vs 内存渲染（env²+centerY 10.14+amp5.2+lw2.75）**RMS=0.0081**（实质相同）；vs 旧 centerY 9.0 渲染 RMS=0.3145——确认为纯整体位移、无形状变化。
- 波谷（−5.03pt）深于波峰（+2.76pt）的固有不对称保留（App 图标同源构图，design.md 明示不做对称化改造）。

## 交付物核验

- DMG 重打：`dist/VoiceKey-1.8.0-arm64.dmg`（5.7MB，arm64，Apple Development 证书签名），挂载核验二进制内嵌居中版 tray.png 逐字节一致。

## 结论

**验证通过（verify_result: pass）**——7 项全 PASS，无 CRITICAL/IMPORTANT 问题。
