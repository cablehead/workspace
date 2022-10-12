use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Args {
    #[clap(value_parser)]
    path: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Put {
        #[clap(short, long, action, help = "Stream stdin, putting an item per line")]
        follow: bool,
        // todo: xor follow and sse
        #[clap(short, long, value_parser, value_name = "SOURCE-NAME")]
        sse: Option<String>,
        // todo: only available with sse
        #[clap(long)]
        last_id: bool,
    },

    Cat {
        #[clap(short, long, action)]
        follow: bool,
        #[clap(long, action)]
        sse: bool,
        #[clap(short, long, value_parser)]
        last_id: Option<i64>,
    },

    Pipe {
        #[clap(value_parser)]
        id: i64,
        #[clap(value_parser)]
        command: String,
        #[clap(value_parser)]
        args: Vec<String>,
    },
}

fn put_one(
    conn: &sqlite::Connection,
    data: String,
    source: Option<String>,
    source_id: Option<i64>,
) {
    let data = data.trim();
    let mut q = conn
        .prepare("INSERT INTO stream (data, source, source_id) VALUES (?, ?, ?)")
        .unwrap()
        .bind(1, data.as_bytes())
        .unwrap()
        .bind(3, source_id)
        .unwrap();
    if let Some(source) = source {
        q = q.bind(2, Some(source.as_bytes())).unwrap();
    }
    q.next().unwrap();
}

fn main() {
    let args = Args::parse();
    let conn = sqlite::open(&args.path).unwrap();
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS stream (
        id INTEGER PRIMARY KEY,
        data INT NOT NULL,
        source TEXT,
        source_id INT
    )",
    )
    .unwrap();
    match &args.command {
        Commands::Put {
            follow,
            sse,
            last_id,
        } => {
            if *follow {
                for line in std::io::stdin().lock().lines() {
                    put_one(&conn, line.unwrap(), None, None);
                }
                return;
            }

            if let Some(sse) = sse {
                if *last_id {
                    let mut q = conn
                        .prepare("SELECT source_id FROM stream WHERE source = ? ORDER BY id DESC LIMIT 1")
                        .unwrap()
                        .bind(1, sse.as_bytes())
                        .unwrap();
                    if let sqlite::State::Done = q.next().unwrap() {
                        println!("no match");
                        return;
                    }
                    let id = q.read::<i64>(0).unwrap();
                    println!("{}", id);
                    return;
                }

                let mut stdin = BufReader::new(std::io::stdin());
                while let Some(event) = parse_sse(&mut stdin) {
                    put_one(&conn, event.data, Some(sse.to_string()), event.id);
                }
                return;
            }

            let mut data = String::new();
            std::io::stdin().read_to_string(&mut data).unwrap();
            put_one(&conn, data, None, None);
        }

        Commands::Cat {
            follow,
            sse,
            last_id,
        } => {
            let mut last_id = last_id.unwrap_or(0);

            // send a comment to establish the connection
            if *sse {
                println!(": welcome");
            }

            loop {
                let mut q = conn
                    .prepare("SELECT id, data FROM stream WHERE id > ? ORDER BY id ASC")
                    .unwrap()
                    .bind(1, last_id)
                    .unwrap();
                while let sqlite::State::Row = q.next().unwrap() {
                    last_id = q.read(0).unwrap();
                    let data = q.read::<String>(1).unwrap();

                    match sse {
                        true => {
                            println!("id: {}", last_id);
                            let data = data.trim().replace("\n", "\ndata: ");
                            println!("data: {}\n", data);
                        }
                        false => println!("{}", data),
                    }
                }
                if !follow {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        Commands::Pipe { id, command, args } => {
            let mut q = conn
                .prepare("SELECT data FROM stream WHERE id = ?")
                .unwrap()
                .bind(1, *id)
                .unwrap();
            if let sqlite::State::Done = q.next().unwrap() {
                println!("no match");
                return;
            }
            let data = q.read::<String>(0).unwrap();

            let mut p = std::process::Command::new(command)
                .args(args)
                .stdin(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            {
                let mut stdin = p.stdin.take().unwrap();
                stdin.write_all(data.as_bytes()).unwrap();
            }
            let res = p.wait_with_output().unwrap();
            std::process::exit(res.status.code().unwrap());
        }
    }
}

#[derive(Debug, PartialEq)]
struct Event {
    data: String,
    event: Option<String>,
    id: Option<i64>,
}

fn parse_sse<R: Read>(buf: &mut BufReader<R>) -> Option<Event> {
    let mut line = String::new();

    let mut data = Vec::<String>::new();
    let mut id: Option<i64> = None;

    loop {
        line.clear();
        let n = buf.read_line(&mut line).unwrap();
        if n == 0 {
            // stream interrupted
            return None;
        }

        if line == "\n" {
            // end of event, emit
            break;
        }

        let (field, rest) = line.split_at(line.find(":").unwrap() + 1);
        let rest = rest.trim();
        match field {
            // comment
            ":" => (),
            "id:" => id = Some(rest.parse::<i64>().unwrap()),
            "data:" => data.push(rest.to_string()),
            _ => todo!(),
        };
    }

    return Some(Event {
        data: data.join(" "),
        event: None,
        id: id,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_sse() {
        let mut buf = BufReader::new(
            indoc! {"
        : welcome
        id: 1
        data: foo
        data: bar

        id: 2
        data: hai

        "}
            .as_bytes(),
        );

        let event = parse_sse(&mut buf).unwrap();
        assert_eq!(
            event,
            Event {
                data: "foo bar".into(),
                event: None,
                id: Some(1),
            }
        );

        let event = parse_sse(&mut buf).unwrap();
        assert_eq!(
            event,
            Event {
                data: "hai".into(),
                event: None,
                id: Some(2),
            }
        );
    }
}
