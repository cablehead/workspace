import AppKit

let pasteboard = NSPasteboard.general
var changeCount = pasteboard.changeCount

while true {
    let timer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { _ in
        if changeCount != pasteboard.changeCount {
            let sourceApp: NSRunningApplication? = NSWorkspace.shared.frontmostApplication
            let source = sourceApp?.bundleIdentifier ?? "unknown"

            var row = [
                "change": pasteboard.changeCount,
                "source": source as Any,
            ]
            var types = [String: String]()

            pasteboard.pasteboardItems?.forEach { item in
                item.types.forEach { x in
                    if let data = pasteboard.data(forType: x) {
                        types[x.rawValue] = data.base64EncodedString()
                    }
                }
            }

            row["types"] = types
            var data = try! JSONSerialization.data(withJSONObject: row)
            data.append("\n".data(using: .utf8)!)
            FileHandle.standardOutput.write(data)
        }
        changeCount = pasteboard.changeCount
    }

    let runLoop = RunLoop.main
    runLoop.add(timer, forMode: .default)
    runLoop.run()
}
