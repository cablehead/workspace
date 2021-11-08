import AppKit

let pasteboard = NSPasteboard.general
var changeCount = 0

while true {
    let timer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { _ in

    if changeCount != pasteboard.changeCount {
        let sourceApp: NSRunningApplication? = NSWorkspace.shared.frontmostApplication
        pasteboard.pasteboardItems?.forEach({ item in
            item.types.forEach({ x in
            let d = pasteboard.data(forType: x)
            print(sourceApp?.bundleIdentifier as Any, x, d!.count)
            })
        })
    }
    changeCount = pasteboard.changeCount
    }

    let runLoop = RunLoop.main
    runLoop.add(timer, forMode: .default)
    runLoop.run()
}
