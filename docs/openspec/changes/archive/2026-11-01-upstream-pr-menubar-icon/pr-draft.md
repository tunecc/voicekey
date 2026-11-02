# PR 草稿：菜单栏图标

## 标题

```
feat(macos): 菜单栏图标改为 App 风格单线声波
```

## 描述

```markdown
## 动机

当前 macOS 菜单栏图标在空闲 / 录音中两套图标间切换（tray.png / tray-active.png）。两套小尺寸像素图标风格不易保持一致，双态切换在菜单栏也缺少辨识价值。本 PR 将菜单栏图标统一为与 App 图标一致的单线声波模板图标，恒定单一形态。

## 改动

- 新增 `Scripts/tray-icons.swift`：模板图标生成脚本（44px/18pt，黑色 + alpha 绘制，由 macOS 按模板规则自动适配浅色/深色菜单栏）。`swift Scripts/tray-icons.swift write f [资源目录]` 输出 tray.png；`preview` 模式可输出候选选型预览图
- 替换 `app/src-tauri/icons/tray.png` 为单线声波图标（App 图标原形包络，含波形墨迹垂直居中补偿）
- 删除 `app/src-tauri/icons/tray-active.png`
- `ui.rs`：`tray_icon()` 去掉 `active` 参数恒用单一图标；移除 `set_tray_active()` 与 `TRAY_ACTIVE`
- `controller.rs`：移除 3 处 `ui::set_tray_active(...)` 调用

## 行为变化

- macOS 菜单栏图标不再随录音状态切换，恒显示单一模板图标

## 验证

- `npm run build`（vite）通过
- `cargo +nightly check --workspace` / `cargo +nightly test --workspace` 通过（基于 v1.9.0 基线的独立 worktree 验证）

## 附注

- `crates/wtlocal/src/kernels.rs` 使用了 unstable 的 `stdarch_neon_dotprod` 特性（`vdotq_s32`），stable ≥1.95 本地编译报 E0658，本 PR 验证使用 nightly 工具链。该问题为上游既有情况、与本 PR 无关，未顺手修改
- 后续调整图标可直接修改 `Scripts/tray-icons.swift` 重新生成素材
```

## 关联

- fork 侧来源提交：`23e2dad`（feat）→ `e3d6f46`（合并单图标）→ `71f2481`（移除双态接线）→ `ac33336`（居中修正）
- change：upstream-pr-menubar-icon（tweak）
