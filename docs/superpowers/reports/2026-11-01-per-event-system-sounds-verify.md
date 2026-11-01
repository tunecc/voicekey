# 验证报告：per-event-system-sounds

- 日期：2026-11-01
- 模式：full（delta spec 存在：audio-feedback MODIFIED 3 条需求）
- 基线：a6ff886 → HEAD

## Summary

| 维度 | 状态 |
| --- | --- |
| Completeness | 6/6 任务完成 |
| Correctness | workspace 测试 56 passed / 0 failed（6 个固定映射/池内 id 测试按新模型重写）；全部交互实机验证通过 |
| Coherence | delta spec 三条需求与实现一致；场景名全保留 |

## 检查项结果

| # | 检查项 | 结果 | 证据 |
| --- | --- | --- | --- |
| 1 | tasks.md 全部完成 | PASS | comet task-complete 6/6 |
| 2 | 实现符合 design | PASS | SYSTEM_SOUNDS 14 项；逐事件取值；SIMPLE_SCHEME/simple_scheme_system_name/simple_scheme_rows 移除；Windows 同名近似音兜底 |
| 3 | delta spec 场景对应 | PASS | validate 通过（仅中文 SHALL 措辞 WARNING）；8+8+2 场景名全保留 |
| 4 | 测试 | PASS | cargo test --workspace 56/0 |
| 5 | 前端构建 | PASS | npm run build ✓ |
| 6 | 实机验证 | PASS | 见下方交互清单 |

## 实机验证清单（用户逐项确认）

- 四行逐事件选择器（「不发声」+ 14 个 macOS 系统铃声），默认 Tink/Pop/不发声/不发声
- 识别处理在进入识别时发声（音量恢复后）；完成/失败默认不发声
- 「不发声」时试听按钮**置灰可点不可用**（按反馈从隐藏改为置灰）
- 改选后试听即所得、会话发声即时跟随
- 验证轮新增三项（范围确认经 propose/decide-escalation continue）：
  1. 失败默认从不发声前的 Basso 改为不发声（用户最终决策）
  2. 提示音会话代数门闩：快速连续开始/停止的延迟提示音不再错拍重叠
  3. NSSound 按音名缓存保活 + stop/play 强制重播：连点试听不再怪声、立即打断从头重播

## 遗留与说明

- Basso 低频在笔记本扬声器偏轻（用户知情选择保留可选）
- Windows 非默认选择静音（已知平台限制，spec 已记录）
- 4 个旧模型测试（固定映射/池内 id 门控/「无」门控）随 spec 更新移除或重写
