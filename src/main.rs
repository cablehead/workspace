use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::os::unix::io::AsRawFd;

use clap::{App, Arg};

use scopeguard;

use termion::event::Key;
use termion::input::TermRead;

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

    let mut guarded = scopeguard::guard(f, |f| {
        termios::tcsetattr(f.as_raw_fd(), termios::TCSANOW, &prev_ios).unwrap();
    });

    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin).lines().map(|x| x.unwrap());

    let mut stdout = io::stdout();

    let mut options = Vec::new();
    let mut cursor = 0;

    if let Some(line) = stdin.next() {
        options.push(line);
        writeln!(stdout, "{}", options[cursor]).unwrap();
    } else {
        return Ok(());
    }

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

    Ok(())
}
