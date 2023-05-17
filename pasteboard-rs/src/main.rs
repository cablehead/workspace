use cocoa::appkit::{NSPasteboard, NSRunningApplication, NSWorkspace};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSData, NSDictionary, NSRunLoop, NSString};
use serde_json::json;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let pasteboard = unsafe { NSPasteboard::generalPasteboard(nil) };
    let mut change_count = unsafe { pasteboard.changeCount() };

    loop {
        thread::sleep(Duration::from_millis(100));

        let new_change_count = unsafe { pasteboard.changeCount() };
        if change_count != new_change_count {
            let source_app = unsafe { NSWorkspace::sharedWorkspace(nil).frontmostApplication() };
            let source = if source_app != nil {
                unsafe { source_app.bundleIdentifier().UTF8String() }
            } else {
                "unknown"
            };

            let mut row = json!({
                "change": new_change_count,
                "source": source,
            });

            let mut types = json!({});

            let pasteboard_items = unsafe { pasteboard.pasteboardItems() };
            if pasteboard_items != nil {
                let count = unsafe { pasteboard_items.count() };
                for i in 0..count {
                    let item = unsafe { pasteboard_items.objectAtIndex(i) };
                    let item_types = unsafe { item.types() };
                    let item_types_count = unsafe { item_types.count() };

                    for j in 0..item_types_count {
                        let x = unsafe { item_types.objectAtIndex(j) };
                        let data = unsafe { pasteboard.dataForType(x) };

                        if data != nil {
                            let base64_encoded = unsafe { data.base64EncodedStringWithOptions(0) };
                            let key = unsafe { x.UTF8String() };
                            let value = unsafe { base64_encoded.UTF8String() };
                            types[key] = value;
                        }
                    }
                }
            }

            row["types"] = types;
            let data = serde_json::to_string(&row).unwrap();
            print!("{}\n", data);
            std::io::stdout().flush().unwrap();
        }

        change_count = new_change_count;
    }
}
