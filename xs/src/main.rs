use std::io::Read;
use std::io::Write;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use clap::{Parser, Subcommand, AppSettings};
use rusqlite;
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Args {
    #[clap(value_parser)]
    path: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Add item
    Add {
        #[clap(value_parser)]
        topic: String,
        #[clap(value_parser)]
        data: String,
    },

    // List items
    List {},

    Replay {
        #[clap(value_parser)]
        id: i32,
    },

    // Process a given stream item with a command, and save the result as a new stream item
    Run {
        #[clap(value_parser)]
        id: i32,
        #[clap(value_parser)]
        topic: String,
        #[clap(value_parser)]
        command: String,
        #[clap(value_parser)]
        args: Vec<String>,
    },

    // Poll for new items
    Poll {
        #[clap(value_parser)]
        id: i32,
    },

    Call {
        #[clap(value_parser)]
        topic: String,
        #[clap(value_parser)]
        response: String,
    },

    Map {
        #[clap(value_parser)]
        topic: String,
        #[clap(value_parser)]
        response: String,
        #[clap(value_parser)]
        command: String,
        #[clap(value_parser)]
        args: Vec<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: i32,
    topic: String,
    stamp: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    err: Option<String>,
    code: i32,
}

pub fn as_base64<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(v) => s.serialize_str(&base64::encode(v)),
        None => s.serialize_none(),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::open(args.path)?;

    create(&conn)?;

    match &args.command {
        Commands::Add { topic, data } => {
            add(
                &conn,
                &topic,
                None,
                None,
                &data,
                &None,
                0,
            )?;
        }
        Commands::List {} => {
            list(&conn)?;
        }
        Commands::Replay { id } => {
            replay(&conn, &id)?;
        }
        Commands::Run {
            id,
            topic,
            command,
            args,
        } => {
            run(&conn, &id, &topic, &command, &args)?;
        }
        Commands::Poll { id } => {
            poll(&conn, &id)?;
        }
        Commands::Call { topic, response } => {
            call(&conn, &topic, &response)?;
        }
        Commands::Map {
            topic,
            response,
            command,
            args,
        } => {
            map(&conn, &topic, &response, &command, &args)?;
        }
    }

    Ok(())
}

fn create(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stream (
           id INTEGER PRIMARY KEY,
           topic TEXT NOT NULL,
           stamp BLOB NOT NULL,
           source_id INTEGER,
           parent_id INTEGER,
           data TEXT,
           err TEXT,
           code INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(())
}

fn add(
    conn: &Connection,
    topic: &String,
    source_id: Option<i32>,
    parent_id: Option<i32>,
    data: &String,
    err: &Option<String>,
    code: i32,
) -> Result<i64> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let id = conn
        .prepare(
            "INSERT INTO stream
        (topic, stamp, source_id, parent_id, data, err, code)
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?
        .insert(params![
            &topic.to_string(),
            stamp.to_le_bytes(),
            source_id,
            parent_id,
            data,
            err,
            code,
        ])?;
    Ok(id)
}

fn create_item(row: &Row) -> rusqlite::Result<Item> {
    Ok(Item {
        id: row.get(0)?,
        topic: row.get(1)?,
        stamp: u128::from_le_bytes(row.get(2)?),
        source_id: row.get(3)?,
        parent_id: row.get(4)?,
        data: row.get(5)?,
        err: row.get(6)?,
        code: row.get(7)?,
    })
}

fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select * from stream;")?;
    let items = stmt.query_map([], create_item)?;
    for item in items {
        println!("{}", serde_json::to_string(&item.unwrap())?);
    }
    Ok(())
}

fn replay(conn: &Connection, id: &i32) -> Result<()> {
    let mut stmt = conn.prepare("select * from stream where id = ?1 limit 1;")?;
    let item = stmt.query_row([id], create_item)?;
    if let Some(data) = item.data {
        std::io::stdout().write_all(&data.as_bytes())?;
    }
    if let Some(err) = item.err {
        std::io::stderr().write_all(&err.as_bytes())?;
    }
    std::process::exit(item.code);
    Ok(())
}

fn run(
    conn: &Connection,
    id: &i32,
    topic: &String,
    command: &String,
    args: &Vec<String>,
) -> Result<()> {
    let mut stmt = conn.prepare("select * from stream where id = ?1 limit 1;")?;
    let item = stmt.query_row([id], create_item)?;
    if item.code != 0 {
        println!("code=={} TODO: output err", item.code);
        std::process::exit(item.code);
    }

    let mut p = process::Command::new(command)
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;
    {
        let mut stdin = p.stdin.take().unwrap();
        stdin.write_all(&item.data.unwrap().as_bytes())?;
    }

    let res = p.wait_with_output()?;

    add(
        conn,
        topic,
        item.source_id.or(Some(item.id)),
        Some(item.id),
        &String::from_utf8(res.stdout)?,
        &Some(String::from_utf8(res.stderr)?),
        res.status.code().unwrap(),
    )?;

    Ok(())
}

fn poll(conn: &Connection, id: &i32) -> Result<()> {
    let mut stmt = conn.prepare("select * from stream where id > ?1 limit 1;")?;
    loop {
        match stmt.query_row([id], create_item) {
            Ok(item) => {
                println!("{:?}", item);
                break;
            }
            Err(err) => match err {
                rusqlite::Error::QueryReturnedNoRows => {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                _ => return Err(err).map_err(anyhow::Error::from),
            },
        };
    }

    Ok(())
}

fn call(conn: &Connection, topic: &String, response: &String) -> Result<()> {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data)?;
    let id = add(&conn, &topic, None, None, &data, &None, 0)?;

    let mut stmt =
        conn.prepare("select * from stream where topic = ? and source_id = ? limit 1;")?;
    loop {
        match stmt.query_row(params![response, id], create_item) {
            Ok(item) => {
                if let Some(data) = item.data {
                    std::io::stdout().write_all(&data.as_bytes())?;
                }
                if let Some(err) = item.err {
                    std::io::stderr().write_all(&err.as_bytes())?;
                }
                std::process::exit(item.code);
                break;
            }
            Err(err) => match err {
                rusqlite::Error::QueryReturnedNoRows => {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                _ => return Err(err).map_err(anyhow::Error::from),
            },
        };
    }

    Ok(())
}

fn poll_topic(conn: &Connection, topic: &String, last_id: &i32) -> Result<Item> {
    loop {
        let mut stmt = conn
            .prepare("select * from stream where topic = ? and id > ? order by id asc limit 1;")?;
        let res = stmt.query_map(params![topic, last_id], create_item)?.next();
        if let Some(row) = res {
            return Ok(row.unwrap());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn map(
    conn: &Connection,
    topic: &String,
    response: &String,
    command: &String,
    args: &Vec<String>,
) -> Result<()> {
    let mut stmt =
        conn.prepare("select * from stream where topic = ? order by source_id desc limit 1;")?;
    let res = stmt.query_map(params![response], create_item)?.next();
    let mut last_id = match res {
        Some(row) => row.unwrap().source_id,
        None => None,
    }
    .unwrap_or(0);
    loop {
        let item = poll_topic(&conn, &topic, &last_id)?;
        run(&conn, &item.id, &response, &command, &args)?;
        last_id = item.id;
    }
}
