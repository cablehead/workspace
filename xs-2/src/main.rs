use std::io::Read;

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
    Put {},
    Cat {
        #[clap(short, long, action)]
        follow: bool,

        #[clap(long, action)]
        sse: bool,

        #[clap(short, long, value_parser)]
        last_id: Option<i64>,
    },
}

fn main() {
    let args = Args::parse();
    let conn = sqlite::open(&args.path).unwrap();
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS stream (
        id INTEGER PRIMARY KEY,
        data INT NOT NULL
    )",
    )
    .unwrap();
    match &args.command {
        Commands::Put {} => {
            let mut data = String::new();
            std::io::stdin().read_to_string(&mut data).unwrap();
            let data = data.trim();
            let mut q = conn
                .prepare("INSERT INTO stream (data) VALUES (?)")
                .unwrap()
                .bind(1, data.as_bytes())
                .unwrap();
            q.next().unwrap();
        }

        Commands::Cat { follow, sse, last_id } => {
            let mut last_id = last_id.unwrap_or(0);
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
    }
}
