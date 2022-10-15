use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

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

        #[clap(long, value_parser)]
        source_id: Option<i64>,
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
    source_id: Option<i64>,
    source: Option<String>,
    parent_id: Option<i64>,
    topic: Option<String>,
    attribute: Option<String>,
    data: String,
) {
    let stamp: Vec<u8> = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_le_bytes()
        .try_into()
        .unwrap();

    let data = data.trim();
    let mut q = conn
        .prepare(
            "INSERT INTO stream (
                source, source_id, parent_id, topic, attribute, data, stamp
           ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .unwrap()
        .bind(1, source.as_ref().map(|x| x.as_bytes()))
        .unwrap()
        .bind(2, source_id)
        .unwrap()
        .bind(3, parent_id)
        .unwrap()
        .bind(4, topic.as_ref().map(|x| x.as_bytes()))
        .unwrap()
        .bind(5, attribute.as_ref().map(|x| x.as_bytes()))
        .unwrap()
        .bind(6, data.as_bytes())
        .unwrap()
        .bind(7, &*stamp)
        .unwrap();
    q.next().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<String>,
    data: String,
    stamp: u128,
}

fn main() {
    let args = Args::parse();
    let conn = sqlite::open(&args.path).unwrap();
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS stream (
        id INTEGER PRIMARY KEY,
        source_id INTEGER,
        source TEXT,
        parent_id INTEGER,
        topic TEXT,
        attribute TEXT,
        data TEXT NOT NULL,
        stamp BLOB NOT NULL
    )",
    )
    .unwrap();
    match &args.command {
        Commands::Put {
            follow,
            sse,
            last_id,
            source_id,
        } => {
            if *follow {
                for line in std::io::stdin().lock().lines() {
                    put_one(&conn, None, None, None, None, None, line.unwrap());
                }
                return;
            }

            if let Some(sse) = sse {
                if *last_id {
                    let mut q = conn
                        .prepare(
                            "
                            SELECT source_id
                            FROM stream
                            WHERE source = ?
                            ORDER BY id DESC
                            LIMIT 1",
                        )
                        .unwrap()
                        .bind(1, sse.as_bytes())
                        .unwrap();
                    if let sqlite::State::Done = q.next().unwrap() {
                        println!("0");
                        return;
                    }
                    let id = q.read::<i64>(0).unwrap();
                    println!("{}", id);
                    return;
                }

                let mut stdin = BufReader::new(std::io::stdin());
                while let Some(event) = parse_sse(&mut stdin) {
                    put_one(
                        &conn,
                        event.id,
                        Some(sse.to_string()),
                        None,
                        None,
                        None,
                        event.data,
                    );
                }
                return;
            }

            let mut data = String::new();
            std::io::stdin().read_to_string(&mut data).unwrap();
            put_one(&conn, *source_id, None, None, None, None, data);
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
                    .prepare(
                        "SELECT
                            id, source_id, topic, data, stamp
                        FROM stream
                        WHERE id > ?
                        ORDER BY id ASC",
                    )
                    .unwrap()
                    .bind(1, last_id)
                    .unwrap();
                while let sqlite::State::Row = q.next().unwrap() {
                    last_id = q.read(0).unwrap();

                    let item = Item {
                        id: last_id,
                        source_id: q.read::<Option<i64>>(1).unwrap(),
                        source: None,
                        parent_id: None,
                        topic: q.read::<Option<String>>(2).unwrap(),
                        attribute: None,
                        data: q.read::<String>(3).unwrap(),
                        stamp: u128::from_le_bytes(
                            q.read::<Vec<u8>>(4).unwrap().try_into().unwrap(),
                        ),
                    };

                    match sse {
                        true => {
                            println!("id: {}", item.id);
                            let data = item.data.trim().replace("\n", "\ndata: ");
                            println!("data: {}\n", data);
                        }

                        false => println!("{}", serde_json::to_string(&item).unwrap()),
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
