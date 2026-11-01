# Brainstorm Summary

- Change: sync-upstream-volume-duck
- Date: 2026-11-01
- 状态：已确认（2026-11-01，用户选择「确认本设计」）

## 确认的技术方案

沿用 Open 阶段 design.md 的全部决策，新增一条集成规则：

1. merge commit 方式合并 `upstream/main`（fork 历史惯例），9 个交集文件按 design.md 决策表取舍：上游音量实现优先、fork 特有功能原样保留
2. 整体删除 fork 固定值音量实现（`crates/core/src/audio/volume.rs`、`Settings.volume`、旧 UI 行、调用点），采纳上游 `OutputVolumeDucker`（默认关闭、20–100% 按比例、默认 70%）
3. 麦克风修复（同一 bug 的同一修法）以上游 #7 文本为准，删除 fork #1 重复实现，保留 fork 的 `devices::running()` 与 live 测试
4. **新增集成规则**：上游在 `stop_recording` 单点还原音量；合并后核对 fork 的全部录音结束路径（Esc 中止、多渠道挑选完成、取消预开麦等），发现绕过上游还原点的路径时，把上游的 `set_active(false)` 补挂到该路径，保持上游单点风格
5. `.github/workflows/release.yml` 直接采纳上游（fork 未改过该文件）

## 关键取舍与风险

- 取舍：不做 settings 迁移（无存量用户，serde 忽略旧字段）；不对 #8 行为做改进
- 风险：fork 结束路径遗漏还原 → 任务 4.3 实测覆盖所有结束路径
- 风险：两份麦克风修复细微差异 → 保留 fork live 测试裁决，行为以 spec 为准

## 测试策略

workspace 测试 + clippy + 前端构建 + 实机音量三段验证（开启→压低→还原）+ 旧 settings.json 兼容 + fork 特性冒烟（提示音、菜单栏图标、更新检查）

## Spec Patch

无（specs/recording-volume-ducking/spec.md 已覆盖验收场景）
