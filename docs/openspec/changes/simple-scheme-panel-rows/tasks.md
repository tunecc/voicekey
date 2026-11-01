# Tasks

## 1. 实现

- [x] 1.1 `Audio.svelte`：`EVENTS` 表「识别处理」描述改为「识别完成、音量恢复后（提示结果就绪）」；简洁方案区块将「识别完成 / 识别失败」合并行拆为两行（文案复用 EVENTS，标注不发声）。验证：`npm run build` 通过 <!-- comet-task:321c664a-953e-4044-aefb-a179d244c295 -->
- [x] 1.2 delta spec：`specs/audio-feedback/spec.md`（MODIFIED 提示音设置需求），`openspec validate` 通过 <!-- comet-task:bb5f7957-495a-4a11-a307-925db47a4a4f -->

## 2. 验证

- [ ] 2.1 打包版实机验证：简洁方案下面板显示四行（开始/处理/完成/失败），处理行描述为新时机语义，完成与失败分行且标注不发声。验证：本机实测记录 <!-- comet-task:8b6919c6-df9f-41b4-ab46-5371d34d5005 -->
