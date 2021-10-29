use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::os::unix::io::AsRawFd;

use scopeguard;

use termion::event::Key;
use termion::input::TermRead;

use termios;

fn main() -> io::Result<()> {
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
            _ => {}
        }
    }

    Ok(())
}
