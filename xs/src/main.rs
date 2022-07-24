use clap::{Parser, Subcommand};

use rusqlite::{Connection, Result};

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
    /// Add Cats
    Add {
        #[clap(value_parser)]
        name: String,
    },
    /// List Cats
    List {},
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
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
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        [],
    )?;

    Ok(())
}

fn insert(conn: &Connection, name: &String) -> Result<()> {
    conn.execute(
        "INSERT INTO cats (name, color_id) values (?1, ?2)",
        &[&name.to_string(), "1"],
    )?;

    Ok(())
}

fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}
