# 任务

- [x] 1. 新增 `Scripts/tray-icons.swift`：支持候选预览与正式资产生成（44×44、资源目录格式、空闲/录音两态）。
- [x] 2. 渲染 3 个候选预览（18pt 实际大小，浅色/深色菜单栏），提交用户选定构图。
- [x] 3. 生成并替换 `app/src-tauri/icons/tray.png` 与 `app/src-tauri/icons/tray-active.png`。
- [x] 4. 构建校验（`cargo check/build -p voicekey`，环境不允许时记录原因）并检查尺寸、透明度与两态差异。
