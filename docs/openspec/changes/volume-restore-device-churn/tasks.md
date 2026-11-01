# 任务清单：volume-restore-device-churn

- [x] T1 回归测试（RED）：FakeBackend 模拟恢复中设备 AudioObjectID 重枚举（同 UID 换 ID），断言旧实现音量停留压低值
- [x] T2 mac.rs 新增 `output_volume_by_uid`，`output_volume_for` 直读失败/ID 不匹配时按 UID 回退
- [x] T3 volume.rs：Backend 增加 `resolve_by_uid`，恢复/持有阶段设备重解析 + 有限重试，UID 消失才放弃；所有权让渡不变；新增单测转绿
- [ ] T4 controller.rs：会话结束后的电平表重启延迟出恢复窗口
- [ ] T5 全量验证：`cargo test -p voicekey-platform`、`cargo test -p voicekey-app`、`cargo build`，真机探针复跑还原曲线
- [ ] T6 验证报告并勾选收尾
