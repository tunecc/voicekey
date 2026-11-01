<p align="center"><img src="docs/icon.png" width="128" alt="VoiceKey"></p>

<h1 align="center">VoiceKey（tunecc fork）</h1>

本仓库是 [J3n5en/voicekey](https://github.com/J3n5en/voicekey) 的个人 fork。这里只记录与上游的差异；功能、截图、各平台说明、使用方式和源码构建请以上游 [README](https://github.com/J3n5en/voicekey#readme) 为准。

## 差异一览

| 方面 | 上游 | 本 fork |
|---|---|---|
| macOS 麦克风释放 | 按名称选择麦克风时，语音结束后可能持续占用 | 已修复 |
| 录音时调整音量 | 不支持 | macOS 支持，默认 15% |
| 录音提示音 | 无 | 录音各阶段可播放提示音，极简/自定义两种方案 |
| macOS 菜单栏图标 | 通用麦克风图形 | App 风格单线声波模板图，随菜单栏明暗自动着色 |
| 云端安装包 | Release（需要签名 Secrets） | 手动 Desktop Build（未签名） |
| 云端构建版本号 | `x.y.z` | `0.0.<run_number>` |

## 1. macOS：语音输入结束后释放麦克风

**现象**：选择具体麦克风后，语音输入结束，系统麦克风指示仍常驻，进程持续占用设备。

**根因**：cpal 0.15.3 的 CoreAudio 后端只为非默认设备注册断连监听；监听闭包持有 `Stream` 克隆并存入 `StreamInner`，与 `Arc<StreamInner>` 形成引用环，而 `Stream` 没有 `Drop`。因此录音线程 `drop(stream)` 不会停止 `AudioUnit`。本仓库按名称选设备时走 `host.devices()`，这些设备的 `is_default` 恒为 `false`，每轮录音都会命中该路径。

**修复**：

- 停止录音时先 `stream.pause()` 再 drop，直接停止音频单元；
- 选中的设备就是系统默认输入时改走 `default_input_device()`，避开断连监听；
- 设置页/引导页的输入电平表只跟随窗口焦点采集，失焦即停。

相关文件：`crates/core/src/audio.rs`、`crates/core/src/audio/devices.rs`、`app/src/pages/Audio.svelte`、`app/src/pages/Onboard.svelte`。

实机回归测试（macOS，需麦克风）：

```bash
cargo test -p voicekey-core --lib -- --include-ignored --test-threads=1
```

测试通过 `kAudioDevicePropertyDeviceIsRunning` 断言停止后输入设备不再运行。

## 2. 录音时调整音量（macOS）

设置 → 音频 → **录音时调整音量**：录音开始把系统输出音量设为指定值，结束后恢复原值。默认 15%，设为 0 关闭。

- 仅 macOS；Windows 路径不变。
- 分别处理主声道与分声道；读不到原音量时不修改，避免恢复时写入错误值。
- 正常结束、识别失败、超时、Esc、候选选择、取消等所有结束路径都会恢复。

相关文件：`crates/core/src/audio/volume.rs`、`app/src-tauri/src/controller.rs`、`app/src-tauri/src/settings.rs`、`app/src/pages/Audio.svelte`。

## 3. 录音提示音

设置 → 音频 → **提示音**（默认开启）：录音各阶段播放提示音，音量跟随系统输出。覆盖四种事件：开始录音、识别处理、完成、出错。

- 音效方案：**极简**——各阶段固定音效（macOS 复用系统音 Tink/Pop，其余平台用内置音，完成/错误不发声）；**自定义**——四个事件逐项挑选内置音效。
- 提示音资源随安装包分发（`resources/sounds/*`）；播放失败自动跳过，不影响录音与识别流程。
- 与「录音时调整音量」联动：开始音播完后才把系统音量单次压到目标值，结束后恢复，避免音量往返。

相关文件：`app/src-tauri/src/sound.rs`、`app/src-tauri/src/controller.rs`、`app/src-tauri/src/settings.rs`、`app/src/pages/Audio.svelte`。

## 4. 云端安装包（Desktop Build）

[`desktop-build.yml`](.github/workflows/desktop-build.yml) 是手动触发的构建，不需要签名密钥：

- 入口：Actions → **Desktop Build** → Run workflow（默认 `main`）
- 产物按平台+架构分开：`VoiceKey-macOS-arm64`（仅 Apple Silicon 的 arm64 DMG，在 Intel runner 上交叉编译）、`VoiceKey-Windows-x64`（NSIS `setup.exe` + MSI），构件保留 14 天
- 版本：`0.0.<run_number>`（MSI 不支持非数字预发布段）
- macOS 使用 adhoc 签名、不做公证、不生成 updater 更新包；安装后需解除隔离：

```bash
xattr -dr com.apple.quarantine /Applications/VoiceKey.app
```

注意：

- adhoc 签名会随构建变化，macOS 可能要求重新授予辅助功能/麦克风权限。
- 测试包不含 updater 产物；应用内更新指向本仓库的正式发布（见第 6 节），`0.0.<run_number>` 测试包不会再被上游版本覆盖。
- 正式签名发布使用 [`release.yml`](.github/workflows/release.yml)，需要签名密钥等 Secrets（见第 6 节）。

## 5. 构建兼容性：stable Rust 1.97

上游 `voicekey-wtlocal` 使用的 `vdotq_s32` 仍在不稳定 feature `stdarch_neon_dotprod` 后面，stable Rust 1.97 会拒绝编译。本 fork 将其替换为等价的内联 `sdot` 汇编，并在运行时检测 ARMv8.2 dotprod 支持，缺失时回退标量实现。

因此不需要 `RUSTC_BOOTSTRAP` 即可在 stable 工具链构建。相关文件：`crates/wtlocal/src/kernels.rs`。

## 6. 更新通道与签名密钥

应用内「检查更新」（设置 → 通用，或菜单栏）读取 `app/src-tauri/tauri.conf.json` 的 `plugins.updater`：

- `endpoints`：本仓库 `https://github.com/tunecc/voicekey/releases/latest/download/latest.json`——打 tag 触发 [`release.yml`](.github/workflows/release.yml) 时自动生成，下载地址按 `$GITHUB_REPOSITORY` 指向本仓库
- `pubkey`：本 fork 自己的 minisign 公钥，与发布签名的私钥配对

发版前一次性配置（私钥永不入库，放在维护者本机）：

```bash
# 1. 生成密钥对：~/.tauri/voicekey.key（私钥）与 ~/.tauri/voicekey.key.pub（公钥）
#    --ci 为无口令密钥；如需口令保护，去掉 --ci 并记住口令
npx tauri signer generate -w ~/.tauri/voicekey.key --ci

# 2. 私钥内容配到仓库 Secrets（无口令密钥无需配置口令 Secret）
gh secret set TAURI_SIGNING_PRIVATE_KEY --repo tunecc/voicekey < ~/.tauri/voicekey.key

# 3. 公钥写入 tauri.conf.json 的 plugins.updater.pubkey（内容来自 cat ~/.tauri/voicekey.key.pub）
```

发版验证：合入后打 tag（如 `v1.7.3`；注意 tag 版本号需符合 MSI 规则——预发布段必须纯数字，如 `v1.7.2-1`，不能用 `fork.1` 这类字母段）触发 `release.yml` → Release 产物含 `latest.json` 且下载 URL 指向本仓库 → 已安装新版本的 app 内「检查更新」可发现新版本并完成验签安装；签名校验失败的更新会被拒绝安装，不影响当前版本。

注意：`release.yml` 除签名密钥外还需要既有的 macOS 签名证书 Secrets（`MACOS_CERTIFICATE` / `MACOS_CERTIFICATE_PASSWORD`）；`release` job 依赖全部平台构建成功，任一平台失败（如 Secrets 缺失）都不会发布 Release 和 `latest.json`。

## 安装与上游

- 上游正式安装包：[J3n5en/voicekey Releases](https://github.com/J3n5en/voicekey/releases)
- 本 fork 的测试包：Actions → [Desktop Build](https://github.com/tunecc/voicekey/actions/workflows/desktop-build.yml) 最近一次成功运行
- 上游源码与文档：[J3n5en/voicekey](https://github.com/J3n5en/voicekey)

本 fork 的 `main` 会合并上游发布；上述改动在每次同步后需要重新验证。

## 声明

本项目通过分析输入法客户端协议实现，仅供学习与个人使用，与字节跳动、腾讯、阿里巴巴无关。接口可能随时变更或失效；语音会发送到对应厂商的服务器。
