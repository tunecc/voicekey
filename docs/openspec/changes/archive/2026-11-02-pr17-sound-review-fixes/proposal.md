# Proposal

## Why

上游 PR #17（feat: 会话提示音）收到维护者 J3n5en 的 CHANGES_REQUESTED review（2026-11-01）：4 个必修项阻塞合并，另有 3 个建议项「有余力建议一起处理」。用户明确要求把 7 项全部修复后重新提请审查。逐项核对 PR 分支（feat/session-sound-feedback，4509ac6）代码，全部可定位、可修复，无范围外改动。

## What Changes

必修（review 1–4）：

1. **移除「识别完成」设置行**（review 选项「去掉这一行」）：设置组剩 3 行（开始/处理/失败），同时删除后端 `SoundEvent::Done`、`DEFAULT_DONE`、`sound_done` 字段与 `api.ts` 的 `soundDone`。理由：PR 自身设计理由「识别很快，完成信号由处理音承担」成立，接线会造成与处理音短间隔连响；旧配置中的 `soundDone` 键被 serde 静默忽略，下次保存自动消失，无需迁移。
2. **处理音/失败音仅在本次会话确实压低过音量时等待恢复窗口**：controller 增加会话级 `ducked` 标志——ducking 真正生效（`enabled && recording`）时置位，显式关闭、取消、新会话时复位；`play_after_restore` 仅在置位时等待，否则立即播放。未开「压低系统音量」（默认关）时处理音不再固定晚约 450ms。
3. **非 macOS 音效列表只含有兜底音频的项**：新增平台感知的 `selectable_sounds()`（macOS=14 个系统音全集；非 macOS=有同名兜底 WAV 的 Tink/Pop/Basso），`list_sounds` 改用它；`sanitize` 逐事件校验同步平台化，Windows 上历史配置里的无兜底音自愈回该事件默认值。
4. **`soundEnabled` 默认 false**：serde 缺省键=关，升级用户不再突然出声；与「压低系统音量」默认关闭原则一致（不含首次创建写 true 的分支）。

建议一并处理（review 5–7）：

5. **`cancel()` 递增提示音会话代数**：松手后恢复窗口内取消（Esc/超时）时，已排队的处理音被作废；`abort()` 现有顺序（先 cancel 后 `play_after_restore(Error)`）保证失败音用新代数照常发声。
6. **延迟压音量消息带会话代数**：`start()` 中延迟发送 `VolumeDucking` 的任务捕获当前代数，醒来后校验一致才发送，复用 `play_after_restore` 的门闩模式；消除极短时间内结束→再开始时旧消息压到新会话开始音的错拍。
7. **资源收敛**：删除 7 个未使用的 WAV（confirmation/ding/glass/minimize/pluck/soft-alert/switch），只保留兜底映射实际用到的 tink/pop/error；`LICENSE.md` 表格重写为 3 行并修正默认用途描述；`tauri.conf.json` 的 `resources/sounds/*` 通配不变。

## Capabilities

### New Capabilities

（无——本 change 不改变本仓库（fork main）行为，交付物是上游 PR 分支的修复提交）

### Modified Capabilities

（无——理由同上：本仓库 `docs/openspec/specs` 与 fork main 代码零改动，不建 delta spec；行为契约以 PR #17 描述修订与 review 要求为准）

## Impact

- **交付分支**：`feat/session-sound-feedback`（worktree `.worktrees/pr-sound-feedback`，基线 4509ac6，PR head）
- **代码**（6 处修改 + 资源删减）：
  - `app/src-tauri/src/sound.rs`：删 Done 事件、新增 selectable_sounds
  - `app/src-tauri/src/settings.rs`：删 sound_done、默认关、sanitize 平台化
  - `app/src-tauri/src/controller.rs`：ducked 标志、cancel 代数、延迟压音量代数
  - `app/src-tauri/src/main.rs`：list_sounds 用 selectable_sounds
  - `app/src/lib/api.ts`：删 soundDone
  - `app/src/pages/Audio.svelte`：提示音组剩 3 行
  - `app/src-tauri/resources/sounds/`：删 7 个 WAV、重写 LICENSE.md
- **明确排除**：crates/ 全部、fork main 分支代码、upstream 基线上的其他功能（Jev、Android 等）、PR 描述中的既有附注（wtlocal 编译问题另案）
- **验证**：cargo（nightly，上游基线 stable ≥1.95 无法编译 wtlocal）check/test、vite 构建、macOS 打包实机试听（用户配合）；Windows 为代码级保证，PR 附注说明
- **交付方式**：push 到 origin/feat/session-sound-feedback（PR #17 自动更新）+ 修订 PR 描述中「识别完成」相关表述 + 回复 review（归档阶段确认）
