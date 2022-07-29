use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand};

use rusqlite;
use rusqlite::{params, types, Connection, Row};

use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(value_parser)]
    path: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add item
    Add {
        #[clap(value_parser)]
        topic: String,
        #[clap(value_parser)]
        data: String,
    },
    /// List items
    List {},
    Run {
        #[clap(value_parser)]
        id: i32,
    },
}

#[derive(Debug)]
struct Item {
    id: i32,
    topic: String,
    stamp: u128,
    source_id: Option<i32>,
    parent_id: Option<i32>,
    data: Option<String>,
    err: Option<String>,
    code: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::open(args.path)?;

    create(&conn)?;

    match &args.command {
        Commands::Add { topic, data } => {
            add(&conn, &topic, &data)?;
        }
        Commands::List {} => {
            list(&conn)?;
        }
        Commands::Run { id } => {
            run(&conn, &id)?;
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

fn add(conn: &Connection, topic: &String, data: &String) -> Result<()> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    conn.execute(
        "INSERT INTO stream
        (topic, stamp, source_id, parent_id, data, err, code)
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &topic.to_string(),
            stamp.to_le_bytes(),
            types::Null,
            types::Null,
            &data.to_string(),
            types::Null,
            0,
        ],
    )?;
    Ok(())
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
        println!("Found item {:?}", item);
    }
    Ok(())
}

fn run(conn: &Connection, id: &i32) -> Result<()> {
    let mut stmt = conn.prepare("select * from stream where id = ?1 limit 1;")?;
    let item = stmt.query_row([id], create_item)?;
    if item.code != 0 {
        println!("code=={} TODO: output err", item.code);
        std::process::exit(item.code);
    }
    println!("{}", (item.data).ok_or(std::fmt::Error)?);
    Ok(())
}
