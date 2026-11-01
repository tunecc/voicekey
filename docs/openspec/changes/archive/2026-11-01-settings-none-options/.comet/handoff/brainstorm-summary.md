# Brainstorm Summary

- Change: settings-none-options
- Date: 2026-11-01

## 确认的技术方案

统一保留字 `none` 作为「无」的可持久化取值（音效四字段与 hold_key）。sanitize 显式保留 `none`，其余未知 id 仍回退默认；serde `#[serde(default)]` 保证旧配置缺省行为不变。`gated_id` 对 `none` 经 `is_known` 为 false 返回 None（不发声）；`hold_codes()` 对 `none` 返回空组合，`set_hold(&[])` 停用长按，点按判定独立不受影响。托盘头部按 hold/tap 是否设置组合生成文案（停用侧不出现、无空按键名；两者皆无时「语音输入」）。前端：事件下拉首项「无」+ 选「无」行隐藏试听；长按下拉「无（停用长按）」+ 卡片与 holdName() 显示「未设置」。

确认依据：用户在 Open 产物确认环节明确选择「两个都确认，继续」（含 design.md D1–D4 同一内容）；Design 阶段复核未产生新决策，AskUserQuestion 未获答复后按推荐项（确认本设计）继续，并如实记录依据。

## 关键取舍与风险

- `none` 保留字 vs 空字符串：选保留字（settings.json 可读、与 `custom` 风格一致、避免与字段缺失混淆）。
- 旧版应用读到 `none` 会按未知值回退默认：仅影响跨版本混用，不损坏配置，不在本次范围。
- `none` 的 play_duration 走 0 时长压音量路径，需测试锁定；托盘四种 hold×tap 组合需单测。

## 测试策略

`cargo test -p voicekey`（sanitize / gated_id / play_duration / hotkey hold=0 / 托盘文案四组合）+ `npm run build` + `node --test app/src/lib/store.test.mjs` + 手动走查 delta specs 全部场景。

## Spec Patch

无——Open 阶段 delta specs（audio-feedback、hotkey-modes）已覆盖全部验收场景。
