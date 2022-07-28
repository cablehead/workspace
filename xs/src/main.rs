use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand};

use rusqlite::{Connection, params};

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
        name: String,
    },
    /// List items
    List {},
}

#[derive(Debug)]
struct Item {
    id: i32,
    topic: String,
    stamp: u128,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::open(args.path)?;

    create(&conn)?;

    match &args.command {
        Commands::Add { name } => {
            insert(&conn, &name)?;
        }
        Commands::List {} => {
            list(&conn)?;
        }
    }

    Ok(())
}

fn create(conn: &Connection) -> Result<()> {
    conn.execute(
        /*
        "CREATE TABLE IF NOT EXISTS stream (
           id INTEGER PRIMARY KEY,
           source_id INTEGER,
           parent_id INTEGER,
           topic TEXT NOT NULL,
           out TEXT,
           err TEXT,
           code INTEGER
        )",
        */
        "CREATE TABLE IF NOT EXISTS stream (
           id INTEGER PRIMARY KEY,
           topic TEXT NOT NULL,
           stamp BLOB NOT NULL
        )",
        [],
    )?;

    Ok(())
}

fn insert(conn: &Connection, name: &String) -> Result<()> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    conn.execute(
        "INSERT INTO stream (topic, stamp) values (?1, ?2)",
        params![&name.to_string(), stamp.to_le_bytes()],
    )?;
    Ok(())
}

fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select * from stream;")?;

    let items = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            topic: row.get(1)?,
            stamp: u128::from_le_bytes(row.get(2)?),
        })
    })?;

    for item in items {
        println!("Found item {:?}", item);
    }

    Ok(())
}
