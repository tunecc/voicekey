// 生成 Tauri 菜单栏模板图标（macOS）。
// 用法：
//   swift Scripts/tray-icons.swift preview [输出目录]
//   swift Scripts/tray-icons.swift write <a-g> [资源目录]
//
// 模板图标由 macOS 按 alpha 着色，因此素材用黑色 + 透明度绘制。
// tray-icon 会把图标缩放到 18pt 高，资产按 44px/18pt 输出。

import AppKit

/// 与现有 44×44 资产保持一致的像素密度（44 / 18 ≈ 2.44x）。
let assetScale: CGFloat = 44.0 / 18.0

enum TrayState: String, CaseIterable {
    case idle
    case active
}

enum Candidate: String, CaseIterable {
    case a, b, c, d
    case e, f, g

    var title: String {
        switch self {
        case .a: return "A · 纯声波三竖条"
        case .b: return "B · 圆角底板 + 声波"
        case .c: return "C · 药丸轮廓 + 声波"
        case .d: return "D · 多层正弦波"
        case .e: return "E · 单线波形（简洁 S）"
        case .f: return "F · 单线波形（App 原形）"
        case .g: return "G · 实心底板 + 镂空波形"
        }
    }

    /// 逻辑点画布；tray-icon 固定按 18pt 高缩放，宽度按比例。
    var canvas: NSSize {
        self == .c ? NSSize(width: 22, height: 18) : NSSize(width: 18, height: 18)
    }
}

/// 当前预览与选型只展示单线波形这一组。
let shortlist: [Candidate] = [.e, .f, .g]

func fillCapsule(_ rect: NSRect, color: NSColor) {
    let path = NSBezierPath(roundedRect: rect, xRadius: rect.width / 2, yRadius: rect.width / 2)
    color.setFill()
    path.fill()
}

func drawCandidate(_ candidate: Candidate, state: TrayState, tint: NSColor) {
    switch candidate {
    case .a:
        let heights: [CGFloat] = state == .idle ? [7.0, 11.0, 7.0] : [10.0, 14.4, 10.0]
        let alpha: CGFloat = state == .idle ? 0.9 : 1.0
        let color = tint.withAlphaComponent(alpha)
        let total: CGFloat = 3 * 2.9 + 2 * 1.9
        var x: CGFloat = 9 - total / 2
        for height in heights {
            fillCapsule(NSRect(x: x, y: 9 - height / 2, width: 2.9, height: height), color: color)
            x += 2.9 + 1.9
        }

    case .b:
        let plate = NSBezierPath(
            roundedRect: NSRect(x: 1.3, y: 1.3, width: 15.4, height: 15.4),
            xRadius: 3.5, yRadius: 3.5
        )
        plate.lineWidth = 1.45
        tint.setStroke()
        plate.stroke()

        let heights: [CGFloat] = state == .idle ? [4.4, 7.4, 4.4] : [6.2, 10.0, 6.2]
        let alpha: CGFloat = state == .idle ? 0.72 : 1.0
        let color = tint.withAlphaComponent(alpha)
        let total: CGFloat = 3 * 1.9 + 2 * 1.35
        var x: CGFloat = 9 - total / 2
        for height in heights {
            fillCapsule(NSRect(x: x, y: 9 - height / 2, width: 1.9, height: height), color: color)
            x += 1.9 + 1.35
        }

    case .c:
        let pill = NSBezierPath(
            roundedRect: NSRect(x: 1.0, y: 3.0, width: 20.0, height: 12.0),
            xRadius: 6, yRadius: 6
        )
        pill.lineWidth = 1.45
        tint.setStroke()
        pill.stroke()

        let heights: [CGFloat] = state == .idle ? [4.0, 6.2, 4.0] : [5.4, 7.4, 5.4]
        let alpha: CGFloat = state == .idle ? 0.72 : 1.0
        let color = tint.withAlphaComponent(alpha)
        let total: CGFloat = 3 * 2.2 + 2 * 1.5
        var x: CGFloat = 11 - total / 2
        for height in heights {
            fillCapsule(NSRect(x: x, y: 9 - height / 2, width: 2.2, height: height), color: color)
            x += 2.2 + 1.5
        }

    case .d:
        let amplitude: CGFloat = state == .idle ? 1.8 : 2.5
        let frontAlpha: CGFloat = state == .idle ? 0.75 : 1.0
        let back = wavePath(
            from: 2.4, to: 15.6, centerY: 10.2, amplitude: amplitude * 0.9, phase: 0.85
        )
        back.lineWidth = 1.55
        tint.withAlphaComponent(frontAlpha * 0.45).setStroke()
        back.stroke()

        let front = wavePath(
            from: 2.4, to: 15.6, centerY: 8.0, amplitude: amplitude, phase: -0.95
        )
        front.lineWidth = 1.85
        tint.withAlphaComponent(frontAlpha).setStroke()
        front.stroke()

    case .e:
        let amplitude: CGFloat = state == .idle ? 3.5 : 5.1
        let alpha: CGFloat = state == .idle ? 0.85 : 1.0
        let line = appWavePath(
            from: 1.7, to: 16.3, centerY: 9.0, amplitude: amplitude,
            cycles: 1.0, phase: .pi + 0.3, envelopePower: 1.0
        )
        line.lineWidth = state == .idle ? 2.35 : 2.7
        tint.withAlphaComponent(alpha).setStroke()
        line.stroke()

    case .f:
        let amplitude: CGFloat = state == .idle ? 3.6 : 5.2
        let alpha: CGFloat = state == .idle ? 0.85 : 1.0
        let line = appWavePath(
            from: 1.7, to: 16.3, centerY: 9.0, amplitude: amplitude,
            cycles: 1.5, phase: 0.6, envelopePower: 1.0
        )
        line.lineWidth = state == .idle ? 2.4 : 2.75
        tint.withAlphaComponent(alpha).setStroke()
        line.stroke()

    case .g:
        let plate = NSBezierPath(
            roundedRect: NSRect(x: 1.4, y: 1.4, width: 15.2, height: 15.2),
            xRadius: 3.5, yRadius: 3.5
        )
        tint.setFill()
        plate.fill()

        let amplitude: CGFloat = state == .idle ? 2.5 : 3.8
        let line = appWavePath(
            from: 3.2, to: 14.8, centerY: 9.0, amplitude: amplitude,
            cycles: 1.5, phase: 0.6, envelopePower: 1.0
        )
        line.lineWidth = state == .idle ? 2.1 : 2.5
        NSGraphicsContext.saveGraphicsState()
        NSGraphicsContext.current?.compositingOperation = .destinationOut
        NSColor.black.setStroke()
        line.stroke()
        NSGraphicsContext.restoreGraphicsState()
    }
}

func appWavePath(
    from x0: CGFloat, to x1: CGFloat, centerY: CGFloat, amplitude: CGFloat,
    cycles: CGFloat, phase: CGFloat, envelopePower: CGFloat
) -> NSBezierPath {
    let path = NSBezierPath()
    let steps = 240
    for index in 0...steps {
        let t = CGFloat(index) / CGFloat(steps)
        let x = x0 + (x1 - x0) * t
        let envelope = pow(sin(.pi * t), envelopePower)
        let y = centerY + sin(2 * .pi * cycles * t + phase) * amplitude * envelope
        if index == 0 {
            path.move(to: NSPoint(x: x, y: y))
        } else {
            path.line(to: NSPoint(x: x, y: y))
        }
    }
    path.lineCapStyle = .round
    path.lineJoinStyle = .round
    return path
}

func wavePath(from x0: CGFloat, to x1: CGFloat, centerY: CGFloat, amplitude: CGFloat, phase: CGFloat) -> NSBezierPath {
    let path = NSBezierPath()
    let steps = 160
    for index in 0...steps {
        let t = CGFloat(index) / CGFloat(steps)
        let x = x0 + (x1 - x0) * t
        let envelope = pow(sin(.pi * t), 0.55)
        let y = centerY + sin(2 * .pi * t + phase) * amplitude * envelope
        if index == 0 {
            path.move(to: NSPoint(x: x, y: y))
        } else {
            path.line(to: NSPoint(x: x, y: y))
        }
    }
    path.lineCapStyle = .round
    path.lineJoinStyle = .round
    return path
}

func render(_ candidate: Candidate, _ state: TrayState, tint: NSColor, scale: CGFloat) -> NSImage {
    let canvas = candidate.canvas
    let pixelWidth = Int((canvas.width * scale).rounded())
    let pixelHeight = Int((canvas.height * scale).rounded())
    guard let rep = NSBitmapImageRep(
        bitmapDataPlanes: nil, pixelsWide: pixelWidth, pixelsHigh: pixelHeight,
        bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
        colorSpaceName: .deviceRGB, bytesPerRow: 0, bitsPerPixel: 0
    ) else {
        fatalError("无法创建位图：\(candidate.rawValue) \(state.rawValue)")
    }

    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
    let context = NSGraphicsContext.current!.cgContext
    context.scaleBy(x: scale, y: scale)
    context.setShouldAntialias(true)
    drawCandidate(candidate, state: state, tint: tint)
    NSGraphicsContext.restoreGraphicsState()

    let image = NSImage(size: canvas)
    image.addRepresentation(rep)
    return image
}

func pngData(_ image: NSImage, pixelWidth: Int, pixelHeight: Int) -> Data {
    guard let tiff = image.tiffRepresentation,
          let rep = NSBitmapImageRep(data: tiff) else {
        fatalError("无法编码 PNG")
    }
    rep.size = NSSize(width: pixelWidth, height: pixelHeight)
    return rep.representation(using: .png, properties: [:])!
}

func write(_ image: NSImage, pixelWidth: Int, pixelHeight: Int, to url: URL) {
    try? FileManager.default.createDirectory(at: url.deletingLastPathComponent(), withIntermediateDirectories: true)
    do {
        try pngData(image, pixelWidth: pixelWidth, pixelHeight: pixelHeight).write(to: url)
    } catch {
        fatalError("写入失败 \(url.path)：\(error)")
    }
}

func pixelSize(_ candidate: Candidate, scale: CGFloat) -> (Int, Int) {
    (Int((candidate.canvas.width * scale).rounded()), Int((candidate.canvas.height * scale).rounded()))
}

// MARK: - 预览

let lightBar = NSColor(calibratedWhite: 0.93, alpha: 1)
let darkBar = NSColor(calibratedWhite: 0.12, alpha: 1)
let lightCard = NSColor(calibratedWhite: 0.97, alpha: 1)
let darkCard = NSColor(calibratedWhite: 0.17, alpha: 1)
let ink = NSColor(calibratedWhite: 0.1, alpha: 1)
let paper = NSColor(calibratedWhite: 0.98, alpha: 1)

func drawBar(_ rect: NSRect, color: NSColor) {
    color.setFill()
    NSBezierPath(roundedRect: rect, xRadius: 5, yRadius: 5).fill()
}

func drawIcon(_ image: NSImage, center: NSPoint, height: CGFloat) {
    let width = height * image.size.width / image.size.height
    image.draw(in: NSRect(x: center.x - width / 2, y: center.y - height / 2, width: width, height: height))
}

func makePreviewSheet() -> NSImage {
    let pointSize = NSSize(width: 1680, height: 560)
    let scale: CGFloat = 2
    let pixelWidth = Int(pointSize.width * scale)
    let pixelHeight = Int(pointSize.height * scale)
    let rep = NSBitmapImageRep(
        bitmapDataPlanes: nil, pixelsWide: pixelWidth, pixelsHigh: pixelHeight,
        bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
        colorSpaceName: .deviceRGB, bytesPerRow: 0, bitsPerPixel: 0
    )!
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
    let context = NSGraphicsContext.current!.cgContext
    context.scaleBy(x: scale, y: scale)

    paper.setFill()
    NSRect(origin: .zero, size: pointSize).fill()

    let titleStyle: [NSAttributedString.Key: Any] = [
        .font: NSFont.systemFont(ofSize: 20, weight: .semibold),
        .foregroundColor: ink,
    ]
    let labelStyle: [NSAttributedString.Key: Any] = [
        .font: NSFont.systemFont(ofSize: 11, weight: .medium),
        .foregroundColor: ink.withAlphaComponent(0.65),
    ]
    let captionStyle: [NSAttributedString.Key: Any] = [
        .font: NSFont.systemFont(ofSize: 12, weight: .regular),
        .foregroundColor: ink.withAlphaComponent(0.78),
    ]

    "VoiceKey 菜单栏图标 · 单线波形候选（模板图标 · 18pt）".draw(
        at: NSPoint(x: 40, y: 520), withAttributes: titleStyle
    )
    "主波形取自 App 图标；上：浅色菜单栏实际大小　中：深色菜单栏实际大小　下：4× 放大".draw(
        at: NSPoint(x: 40, y: 496), withAttributes: captionStyle
    )

    let columns = shortlist.flatMap { candidate in
        TrayState.allCases.map { (candidate, $0) }
    }
    let columnWidth: CGFloat = 230
    let gap: CGFloat = 16
    let left: CGFloat = 40

    for (index, pair) in columns.enumerated() {
        let (candidate, state) = pair
        let x = left + CGFloat(index) * (columnWidth + gap)
        let black = render(candidate, state, tint: .black, scale: assetScale)
        let white = render(candidate, state, tint: .white, scale: assetScale)

        let stateTitle = state == .idle ? "空闲" : "录音中"
        "\(candidate.title.split(separator: " ").first ?? "") \(stateTitle)".draw(
            at: NSPoint(x: x, y: 470), withAttributes: labelStyle
        )

        drawBar(NSRect(x: x, y: 424, width: columnWidth, height: 42), color: lightBar)
        drawIcon(black, center: NSPoint(x: x + columnWidth / 2, y: 445), height: 18)

        drawBar(NSRect(x: x, y: 374, width: columnWidth, height: 42), color: darkBar)
        drawIcon(white, center: NSPoint(x: x + columnWidth / 2, y: 395), height: 18)

        let zoom = render(candidate, state, tint: .black, scale: 8)
        let zoomWhite = render(candidate, state, tint: .white, scale: 8)
        "4× 浅色".draw(at: NSPoint(x: x, y: 354), withAttributes: labelStyle)
        drawBar(NSRect(x: x, y: 230, width: columnWidth, height: 120), color: lightCard)
        drawIcon(zoom, center: NSPoint(x: x + columnWidth / 2, y: 290), height: 54)
        "4× 深色".draw(at: NSPoint(x: x, y: 214), withAttributes: labelStyle)
        drawBar(NSRect(x: x, y: 90, width: columnWidth, height: 120), color: darkCard)
        drawIcon(zoomWhite, center: NSPoint(x: x + columnWidth / 2, y: 150), height: 54)
    }

    "模板图标由 macOS 按 alpha 自动适配浅色/深色菜单栏；录音态通过振幅、线宽与不透明度区分。".draw(
        at: NSPoint(x: 40, y: 40), withAttributes: captionStyle
    )

    NSGraphicsContext.restoreGraphicsState()
    let image = NSImage(size: pointSize)
    image.addRepresentation(rep)
    return image
}

// MARK: - 命令行

let args = Array(CommandLine.arguments.dropFirst())
let command = args.first ?? "preview"

switch command {
case "preview":
    let dir = URL(fileURLWithPath: args.count > 1 ? args[1] : "/tmp/voicekey-tray-preview")
    try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
    for candidate in shortlist {
        for state in TrayState.allCases {
            let image = render(candidate, state, tint: .black, scale: assetScale)
            let (width, height) = pixelSize(candidate, scale: assetScale)
            let url = dir.appendingPathComponent("\(candidate.rawValue)-\(state.rawValue).png")
            write(image, pixelWidth: width, pixelHeight: height, to: url)
            print("wrote \(url.path) (\(width)x\(height))")
        }
    }
    let sheet = makePreviewSheet()
    let sheetURL = dir.appendingPathComponent("contact-sheet.png")
    write(sheet, pixelWidth: 3360, pixelHeight: 1120, to: sheetURL)
    print("wrote \(sheetURL.path) (3360x1120)")

case "write":
    guard args.count > 1, let candidate = Candidate(rawValue: args[1]) else {
        print("用法：swift Scripts/tray-icons.swift write <a-g> [资源目录]")
        exit(1)
    }
    let dir = URL(fileURLWithPath: args.count > 2 ? args[2] : "app/src-tauri/icons")
    for state in TrayState.allCases {
        let image = render(candidate, state, tint: .black, scale: assetScale)
        let (width, height) = pixelSize(candidate, scale: assetScale)
        let name = state == .idle ? "tray.png" : "tray-active.png"
        write(image, pixelWidth: width, pixelHeight: height, to: dir.appendingPathComponent(name))
        print("wrote \(dir.appendingPathComponent(name).path) (\(width)x\(height))")
    }

default:
    print("用法：swift Scripts/tray-icons.swift preview [输出目录] | write <a-g> [资源目录]")
    exit(1)
}
