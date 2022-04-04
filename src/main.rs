use commitlog::*;
use commitlog::message::MessageSet;

fn main() {
    // open a directory called 'log' for segment and index storage
    let opts = LogOptions::new("log");
    let mut log = CommitLog::new(opts).unwrap();

    let s = "hello world";

    // append to the log
    log.append_msg(s).unwrap(); // offset 0
    // log.append("second message").unwrap(); // offset 1

    // read the messages
    let messages = log.read(0, ReadLimit::default()).unwrap();
    for msg in messages.iter() {
        println!("{} - {}", msg.offset(), String::from_utf8_lossy(msg.payload()));
    }

    // prints:
    //    0 - hello world
    //    1 - second message
}
