use cocoa::appkit::{NSPasteboard, NSRunningApplication};
use cocoa::base::{nil};
use cocoa::foundation::{NSArray, NSData, NSString};
use objc::runtime::Object;
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
            let source_app = unsafe { NSRunningApplication::frontmostApplication(nil) };
            let source = if source_app != nil {
                unsafe { NSString::UTF8String(source_app.bundleIdentifier()) }
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
                let count = unsafe { NSArray::count(pasteboard_items as *mut Object) };
                for i in 0..count {
                    let item = unsafe { pasteboard_items.objectAtIndex(i) };
                    let item_types = unsafe { item.types() };
                    let item_types_count = unsafe { NSArray::count(item_types as *mut Object) };

                    for j in 0..item_types_count {
                        let x = unsafe { item_types.objectAtIndex(j) };
                        let data = unsafe { pasteboard.dataForType(x) };

                        if data != nil {
                            let base64_encoded = unsafe { NSData::base64EncodedStringWithOptions(data, 0) };
                            let key = unsafe { NSString::UTF8String(x) };
                            let value = unsafe { NSString::UTF8String(base64_encoded) };
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
