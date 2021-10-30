use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::Write;
use std::iter::Iterator;
use std::os::unix::io::AsRawFd;

use clap::{App, Arg};

use scopeguard;

// use termion::event::Key;
// use termion::input::TermRead;

use termios;

fn main() -> io::Result<()> {
    let matches = App::new("x-select")
        .version("0.0.1")
        .about("interactively explore an input stream")
        .arg(
            Arg::new("keys")
                .short('k')
                .long("capture-keys")
                .about("keys to capture items")
                .takes_value(true)
                .multiple_values(true)
                .use_delimiter(true),
        )
        .get_matches();

    let mut capture_keys = HashSet::new();

    if let Some(keys) = matches.values_of("keys") {
        for k in keys {
            if k.len() > 1 {
                eprintln!("capture keys should be single characters: '{}'", k);
                std::process::exit(1);
            }
            capture_keys.insert(k.chars().next().unwrap());
        }
    }

    let f = fs::OpenOptions::new().read(true).open("/dev/tty")?;

    let mut ios = termios::Termios::from_fd(f.as_raw_fd())?;
    let prev_ios = ios;
    ios.c_lflag &= !(termios::ECHO | termios::ICANON);
    termios::tcsetattr(f.as_raw_fd(), termios::TCSANOW, &ios)?;

    let _guarded = scopeguard::guard(f, |f| {
        termios::tcsetattr(f.as_raw_fd(), termios::TCSANOW, &prev_ios).unwrap();
    });

    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin).lines().map(|x| x.unwrap());

    let mut stdout = io::stdout();

    let mut options = Vec::new();
    let cursor = 0;

    if let Some(line) = stdin.next() {
        options.push(line);
        writeln!(stdout, "{}", options[cursor]).unwrap();
    } else {
        return Ok(());
    }

    /*
    for key in guarded.by_ref().keys() {
        let key = key.unwrap();

        match key {
            Key::Char('q') => {
                break;
            }

            Key::Char(x) if capture_keys.contains(&x) => {
                eprintln!("{}:{}", x, options[cursor]);
                options.remove(cursor);
                if cursor >= options.len() {
                    if let Some(line) = stdin.next() {
                        options.push(line);
                    } else {
                        return Ok(());
                    }
                }
                writeln!(stdout, "{}", options[cursor]).unwrap();
            }

            Key::Left => {
                if cursor == 0 {
                    continue;
                }
                cursor -= 1;
                writeln!(stdout, "{}", options[cursor]).unwrap();
            }

            Key::Right => {
                cursor += 1;
                if cursor >= options.len() {
                    if let Some(line) = stdin.next() {
                        options.push(line);
                    } else {
                        return Ok(());
                    }
                }
                writeln!(stdout, "{}", options[cursor]).unwrap();
            }

            _ => {
                println!("{:?}", key);
            }
        }
    }
    */

    Ok(())
}

struct Stream<R> {
    inner: R,
    store: Vec<String>,
    cursor: usize,
}

impl<R: Iterator<Item = String>> Stream<R> {
    // Returns None if the provided iter has no values
    pub fn from(mut inner: R) -> Option<Stream<R>> {
        // this will block until an initial item has been read
        let s = inner.next()?;

        let store: Vec<String> = vec![s];

        Some(Stream {
            inner: inner,
            store: store,
            cursor: 0,
        })
    }

    // None means the whole input stream has been consumed
    pub fn current(&self) -> Option<&str> {
        Some(&self.store[self.cursor][..])
    }

    // None means we are at the boundary, so the cursor cannot be moved
    pub fn move_right(&mut self) -> Option<&str> {
        if self.cursor < self.store.len() - 1 {
            self.cursor += 1;
            return self.current();
        }

        let next = self.inner.next()?;
        self.store.push(next);
        self.cursor += 1;
        self.current()
    }

    // None means we are at the boundary, so the cursor cannot be moved
    pub fn move_left(&mut self) -> Option<&str> {
        if self.cursor == 0 {
            return None;
        }
        self.cursor -= 1;
        self.current()
    }

    // None means the whole input stream has been consumed
    pub fn remove(&mut self) -> Option<&str> {
        self.store.remove(self.cursor);
        if self.store.len() == 0 {
            let next = self.inner.next()?;
            self.store.push(next);
        }
        self.current()
    }
}

#[test]
fn test_stream_navigation() {
    let mut stdin = io::Cursor::new(Vec::new());
    writeln!(stdin, "one\ntwo\nthree").unwrap();
    stdin.seek(io::SeekFrom::Start(0)).unwrap();
    let stdin = BufReader::new(stdin).lines().map(|x| x.unwrap());

    let mut stream = Stream::from(stdin).expect("stdin has values");

    assert_eq!(stream.current(), Some("one"));
    assert_eq!(stream.move_right(), Some("two"));
    assert_eq!(stream.move_right(), Some("three"));
    assert_eq!(stream.move_right(), None);
    assert_eq!(stream.move_left(), Some("two"));
    assert_eq!(stream.move_left(), Some("one"));
    assert_eq!(stream.move_left(), None);
    assert_eq!(stream.move_right(), Some("two"));
    assert_eq!(stream.move_right(), Some("three"));
    assert_eq!(stream.move_right(), None);
}

#[test]
fn test_stream_remove() {
    let mut stdin = io::Cursor::new(Vec::new());
    writeln!(stdin, "one\ntwo\nthree").unwrap();
    stdin.seek(io::SeekFrom::Start(0)).unwrap();
    let stdin = BufReader::new(stdin).lines().map(|x| x.unwrap());

    let mut stream = Stream::from(stdin).expect("stdin has values");

    assert_eq!(stream.remove(), Some("two"));
    assert_eq!(stream.move_right(), Some("three"));
    assert_eq!(stream.move_left(), Some("two"));
    assert_eq!(stream.remove(), Some("three"));
    assert_eq!(stream.remove(), None);
}
