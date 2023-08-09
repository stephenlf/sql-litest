use std::path::PathBuf;
use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// TEST
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Execute a SQL command
    Run {
        /// A SQL command literal, wrapped in quotes. Ignored if '-f' is specified.
        sql: Option<String>,

        /// Run the contents of the provided .sql file 
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
    
    /// TODO
    Game
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql_path = env::current_exe()?;
    let sql_path = sql_path.parent()
        .expect("Could not find parent folder of this executable.")
        .join("db.db3");

    #[cfg(debug_assertions)]
    println!("Opening database at {:?}", sql_path);

    let conn = Connection::open(PathBuf::from(sql_path))?;

    let cli = Cli::parse();
    if let Cli {command: Some(Commands::Run {sql, file})} = cli {
        if file.is_some() {
            let sql_string = std::fs::read_to_string(file.unwrap())?;
            conn.execute(&sql_string, ())?;
        } else if sql.is_some() {
            conn.execute(&sql.unwrap(), ())?;
        }
    } else {
        println!("Type `sqll help`");
    }
    Ok(())
}
