# Proposal: sync-upstream-v1-7-1

## 变更动机

上游 J3n5en/voicekey 在 fork 点（merge-base 75b0f04）之后新增 3 个提交，本仓库需要同步这些更新，同时完整保留全部本地自研修改（音频反馈提示音、菜单栏图标、麦克风释放、录音音量包络、CI 构建流程等）。

上游新增内容：

| 提交 | 内容 | 性质 |
| ---- | ---- | ---- |
| `cb5e70b` | v1.7.1：`Cargo.toml`、`app/src-tauri/tauri.conf.json` 版本号 1.7.0 → 1.7.1 | 版本号 |
| `7bd0373` | Android 0.5.1（versionCode 7）：`android/app/build.gradle.kts` | 版本号 |
| `e82f110` | Chatterfly：禁用服务端 VAD 结束 utterance（移除 `single_utterance=true`，改由 `"{}"` 文本帧驱动结束，仍保留 is_final 与标点），修复“服务器关闭了连接” | 单文件功能修复 |

## 与本地修改的关系分析

- 两侧自 merge-base 以来唯一同时改动的文件是 `app/src-tauri/tauri.conf.json`：上游改版本号行，本地改 `bundle.resources`（提示音资源声明），位于不同 hunk，git 自动合并。
- 上游唯一的功能修复 `crates/core/src/chatterfly.rs` 属于本地从未修改的模块，与本地自研功能（音效、托盘、麦克风、音量）无交集。
- **结论：上游没有对我们自研的功能做增强或改动，不存在“保留上游还是保留本地”的取舍点**，直接合并并保留双方改动即可。
- 试合并（`git merge --no-commit --no-ff upstream/main`）已验证：自动合并成功，零冲突。

## 目标

1. 将 `upstream/main`（`7bd0373`）合并进 `main`，保留全部本地提交与修改。
2. 合并后 `Cargo.lock` 中 workspace 成员版本与 v1.7.1 一致。
3. 构建与测试通过，确认合并没有破坏本地功能。

## 范围

- 合并引入的上游文件：`Cargo.toml`、`app/src-tauri/tauri.conf.json`、`android/app/build.gradle.kts`、`crates/core/src/chatterfly.rs`
- 随版本号刷新的生成物：`Cargo.lock`（workspace 成员版本字段）
- 不修改任何本地自研代码；不引入 delta spec（本次不改变本仓库规格化行为）。

## 风险

- 低：无冲突合并；上游功能修复与本地修改无语义交集；交付文件 5 个，低于 tweak 阈值。
