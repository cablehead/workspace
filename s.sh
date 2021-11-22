#!/usr/bin/env xcrun swift

import AppKit

let sourceApp: NSRunningApplication? = NSWorkspace.shared.frontmostApplication
let source = sourceApp?.bundleIdentifier ?? "unknown"

if let icon = sourceApp?.icon {

    let imageRep = NSBitmapImageRep(data: icon.tiffRepresentation!)
    let pngData = imageRep?.representation(using: .png, properties: [:])

    var encoded = pngData?.base64EncodedString()
    encoded = "data:image/png;base64," + encoded!
    let row = [
        "source_icon": encoded, ]
    var data = try! JSONSerialization.data(withJSONObject: row)
    data.append("\n".data(using: .utf8)!)
    FileHandle.standardOutput.write(data)

    /*
    let imageRep = NSBitmapImageRep(data: image.tiffRepresentation!)
    let pngData = imageRep?.representation(using: .png, properties: [:])
    FileHandle.standardOutput.write(pngData!)
    */

    /*
    let encoded = icon.tiffRepresentation?.base64EncodedString()
    let row = [
        "source_icon": encoded, ]
    var data = try! JSONSerialization.data(withJSONObject: row)
    data.append("\n".data(using: .utf8)!)
    FileHandle.standardOutput.write(data)
    */
}
