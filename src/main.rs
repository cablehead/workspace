use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::iter::Iterator;
use std::os::unix::io::AsRawFd;

use clap::App;
use clap::Arg;

use scopeguard;

use termion::event::Key;
use termion::input::TermRead;

use termios;

fn main() -> io::Result<()> {
    let matches = App::new("x-select")
        .version("0.0.1")
        .about("interactively explore an input stream")
        .arg(
            Arg::new("keep-keys")
                .short('k')
                .long("keep-keys")
                .about("record the selected item, but keep it in the stream")
                .takes_value(true)
                .multiple_values(true)
                .use_delimiter(true),
        )
        .arg(
            Arg::new("pop-keys")
                .short('p')
                .long("pop-keys")
                .about("record the selected item and remove it from the stream")
                .takes_value(true)
                .multiple_values(true)
                .use_delimiter(true),
        )
        .get_matches();

    let mut keep_keys = HashSet::new();
    let mut pop_keys = HashSet::new();

    // TODO: need to get a test on keep / pop
    if let Some(keys) = matches.values_of("keep-keys") {
        for k in keys {
            if k.len() > 1 {
                eprintln!("capture keys should be single characters: '{}'", k);
                std::process::exit(1);
            }
            keep_keys.insert(k.chars().next().unwrap());
        }
    }

    if let Some(keys) = matches.values_of("pop-keys") {
        for k in keys {
            if k.len() > 1 {
                eprintln!("capture keys should be single characters: '{}'", k);
                std::process::exit(1);
            }
            pop_keys.insert(k.chars().next().unwrap());
        }
    }

    let f = fs::OpenOptions::new().read(true).open("/dev/tty")?;
    let mut ios = termios::Termios::from_fd(f.as_raw_fd())?;
    let prev_ios = ios;
    ios.c_lflag &= !(termios::ECHO | termios::ICANON);
    termios::tcsetattr(f.as_raw_fd(), termios::TCSANOW, &ios)?;
    let mut guarded = scopeguard::guard(f, |f| {
        termios::tcsetattr(f.as_raw_fd(), termios::TCSANOW, &prev_ios).unwrap();
    });

    let stdin = io::stdin();
    let stdin = BufReader::new(stdin).lines().map(|x| x.unwrap());

    let mut stdout = io::stdout();

    let stream = Stream::from(stdin);
    if stream.is_none() {
        return Ok(());
    }
    let mut stream = stream.unwrap();

    writeln!(stdout, "{}", stream.current().unwrap()).unwrap();

    for key in guarded.by_ref().keys() {
        let key = key.unwrap();

        match key {
            Key::Char('q') => {
                break;
            }

            Key::Char(x) if keep_keys.contains(&x) => {
                eprintln!("{}:{}", x, stream.current().unwrap());
            }

            Key::Char(x) if pop_keys.contains(&x) => {
                eprintln!("{}:{}", x, stream.current().unwrap());
                if let Some(item) = stream.remove() {
                    writeln!(stdout, "{}", item).unwrap();
                } else {
                    break;
                }
            }

            Key::Left => {
                if let Some(item) = stream.move_left() {
                    writeln!(stdout, "{}", item).unwrap();
                }
            }

            Key::Right => {
                if let Some(item) = stream.move_right() {
                    writeln!(stdout, "{}", item).unwrap();
                }
            }

            _ => {
                // println!("{:?}", key);
            }
        }
    }

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

        if self.store.len() <= self.cursor {
            self.cursor -= 1;
        }

        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Seek;

    fn stdin(s: &str) -> impl Iterator<Item = String> {
        let mut stdin = io::Cursor::new(Vec::new());
        let _ = stdin.write(s.as_bytes()).unwrap();
        stdin.seek(io::SeekFrom::Start(0)).unwrap();
        BufReader::new(stdin).lines().map(|x| x.unwrap())
    }

    #[test]
    fn stream_from_empty_is_none() {
        assert!(Stream::from(stdin("")).is_none());
    }

    #[test]
    fn stream_basic_navigation() {
        let mut stream = Stream::from(stdin("one\ntwo\nthree")).unwrap();
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
    fn stream_remove_last_more_to_read() {
        let mut stream = Stream::from(stdin("one\ntwo")).unwrap();
        assert_eq!(stream.remove(), Some("two"));
    }

    #[test]
    fn stream_remove_last_no_more_to_read() {
        let mut stream = Stream::from(stdin("one\ntwo")).unwrap();
        assert_eq!(stream.move_right(), Some("two"));
        assert_eq!(stream.remove(), Some("one"));
    }

    #[test]
    fn stream_remove_mid_ok() {
        let mut stream = Stream::from(stdin("one\ntwo\nthree")).unwrap();
        assert_eq!(stream.move_right(), Some("two"));
        assert_eq!(stream.move_right(), Some("three"));
        assert_eq!(stream.move_left(), Some("two"));
        assert_eq!(stream.remove(), Some("three"));
    }

    #[test]
    fn stream_remove_all() {
        let mut stream = Stream::from(stdin("one")).unwrap();
        assert_eq!(stream.remove(), None);
    }
}
