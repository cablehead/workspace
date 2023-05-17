use cocoa::appkit::{NSApp, NSPasteboard, NSPasteboardItem, NSRunningApplication, NSWorkspace};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSDictionary, NSRunLoop, NSString, NSDate};
use std::collections::HashMap;
use std::str;
use std::slice;
use std::thread;
use std::time::Duration;
use serde_json::json;

fn main() {
    unsafe {
        let app = NSApp();
        let pasteboard = NSPasteboard::generalPasteboard(app);
        let mut change_count = pasteboard.changeCount();

        loop {
            let current_change_count = pasteboard.changeCount();
            if change_count != current_change_count {
                let workspace = NSWorkspace::sharedWorkspace(nil);
                let source_app: id = workspace.frontmostApplication();
                let source = if source_app != nil {
                    let bundle_identifier = source_app.bundleIdentifier();
                    NSString::UTF8String(bundle_identifier) as *const u8
                } else {
                    "unknown".as_ptr()
                };

                let source = str::from_utf8(slice::from_raw_parts(source, strlen(source))).unwrap();

                let mut row = json!({
                    "change": current_change_count,
                    "source": source,
                });

                let nsarray_ptr = pasteboard.pasteboardItems();
                let mut types = HashMap::new();

                if nsarray_ptr.count() != 0 {
                    for i in 0..NSArray::count(nsarray_ptr) {
                        let raw_item_ptr = NSArray::objectAtIndex(nsarray_ptr, i);
                        let item = NSPasteboardItem::wrap(raw_item_ptr);
                        let item_types = item.types();

                        for j in 0..NSArray::count(item_types) {
                            let raw_type_ptr = NSArray::objectAtIndex(item_types, j);
                            let ns_type = NSString::wrap(raw_type_ptr);
                            let type_str = ns_type.UTF8String() as *const u8;
                            let type_str = str::from_utf8(slice::from_raw_parts(type_str, ns_type.len())).unwrap();

                            let data = pasteboard.dataForType_(ns_type);
                            if data != nil {
                                let base64_encoded = data.base64EncodedStringWithOptions_(0);
                                let base64_encoded = base64_encoded.UTF8String() as *const u8;
                                let base64_encoded = str::from_utf8(slice::from_raw_parts(base64_encoded, base64_encoded.len())).unwrap();

                                types.insert(type_str.to_string(), base64_encoded.to_string());
                            }
                        }
                    }
                }

                row["types"] = json!(types);
                let data = serde_json::to_string(&row).unwrap();
                println!("{}", data);

                change_count = current_change_count;
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn strlen(ptr: *const u8) -> usize {
    let mut len = 0;
    while unsafe { *ptr.add(len) } != 0 {
        len += 1;
    }
    len
}