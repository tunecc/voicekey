# Tasks

## 1. 实现

- [x] 1.1 `controller.rs`：移除录音结束路径的 `SoundEvent::Processing` 播放；`Ctl` 增加 `stopped_at` 记录（`stop_recording` 写入）；全部结果就绪路径（单候选注入、多候选面板）改为异步「恢复等待 → Processing → 按时长间隔 → Done」序列，失败路径不变。验证：`cargo check --workspace` 通过，grep 无录音结束处的 Processing 播放 <!-- comet-task:1f0055ae-a491-4a51-abc4-01551f96e96e -->
- [x] 1.2 delta spec：创建 `specs/audio-feedback/spec.md`（MODIFIED 会话事件提示音需求），`openspec validate` 通过 <!-- comet-task:dc28dd31-3d52-48ec-8ac6-cd5b91a49fe1 -->

## 2. 验证

- [x] 2.1 `cargo test --workspace` 通过。验证：退出码 0 <!-- comet-task:561c60c8-40d5-4974-acca-6677a33d51c7 -->
- [x] 2.2 打包版实机验证：简洁方案下开始音全音量播放 → 录音 → 结果就绪后「啵」声以恢复后的全音量播放；失败路径无处理音。验证：本机实测记录 <!-- comet-task:c688b8cd-d085-4033-ac28-360e1698c4b0 -->
