# Proposal

## 问题描述

菜单栏托盘图标（`app/src-tauri/icons/tray.png`）的波形墨迹在 18pt 画布中垂直偏低。用户反馈「好像没有居中、有点歪」。

## 根因分析

- 波形公式 `y = centerY + sin(3πt + 0.6)·amp·sin²(πt)` 的上下偏移固有不对称：波谷（t≈0.44）向下探 5.03pt，波峰（t≈0.77）只向上探 2.76pt（amp=5.2）。
- 画布中心 centerY=9.0 取的是波形中线而非**墨迹包围盒中心**，导致墨迹整体下沉。
- 实测（alpha 阈值 0.05）：上边距 ≈4.5pt / 下边距 ≈2.45pt，墨迹中心偏低 ≈1.0pt；公式值 1.14pt（差值来自抗锯齿阈值与线宽半宽的计法）。
- 该不对称与 App 图标（`Scripts/icon.swift`）同源——图标本身也是偏沉构图；本修复只调整托盘在菜单栏中的视觉居中，不改 App 图标。

## 修复目标

- 候选 f 的 `centerY` 由 9.0 调整为 10.14（上移 1.14pt），使墨迹包围盒在 18pt 画布中上下边距相等（各 ≈3.7pt）。
- 重新生成 `tray.png`，波形曲率/振幅/线宽不变（env²、amp 5.2、lw 2.75 保持）。
- 重打本地 DMG 安装包。

## 影响范围

- `Scripts/tray-icons.swift`（1 行参数）
- `app/src-tauri/icons/tray.png`（重新生成）
- 不改：App 图标、波形形状、Rust 代码、spec 行为（无 delta spec，`.openspec.yaml` 设 `skip_specs: true`）
