# Proposal

## Why

设置中凡是用户可自定义的选择项，都应能回到「不使用」的状态，自定义才完整。当前自定义音效方案的四个事件只能从内置音效中选一个，长按说话必须绑定一个按键——用户既不能单独关掉某个事件的提示音，也不能停用长按模式。

## What Changes

- 自定义音效方案下，录音开始 / 识别处理 / 识别完成 / 识别失败四个事件的下拉增加「无」选项：选择后该事件不再发声，其余事件不受影响。
- 「无」是显式且可持久化的取值：选择后重启应用，配置 sanitize 不得把它回退成默认音效。
- 长按说话的按键选择增加「无」：选择后停用长按触发（点按说话保持原行为），设置页与托盘菜单对「未设置」有明确展示。
- 未选择「无」时所有行为与现状一致：默认值不变，提示音总开关与音效方案语义不变。

## Capabilities

### Modified Capabilities

- `audio-feedback`：自定义方案的逐事件音效选择控件必须提供「无」；选中「无」的事件不发声，且该选择随设置持久化。

### New Capabilities

- `hotkey-modes`：长按说话与点按说话两种触发模式可独立启用；长按说话可置「无」停用，停用后点按触发不受影响。

## Impact

- 前端：`app/src/pages/Audio.svelte`（音效下拉加「无」）、`app/src/pages/Keys.svelte`（长按键下拉加「无」与未设置展示）。
- 后端：`app/src-tauri/src/settings.rs`（sanitize 与 `hold_codes`/`hold_name` 支持「无」）、`app/src-tauri/src/sound.rs`（gated_id 对「无」不发声）、`app/src-tauri/src/hotkey.rs`（`Config::set_hold(&[])` 已支持空组合，核对触发链路）。
- 托盘：`app/src-tauri/src/ui.rs` 菜单头对未设置长按键的展示。
- 不修改提示音总开关、音效方案的语义；不改动点按快捷键与麦克风已有的清除/默认项。
