use std::path::PathBuf;
use clap::{Parser, Subcommand};
use rusqlite::{Connection, types::ValueRef};
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
    /// Execute a SQL command and returns the number of rows changed
    Run {
        /// A SQL command literal, wrapped in quotes. Ignored if '-f' is specified.
        sql: Option<String>,

        /// Run the contents of the provided .sql file 
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },

    /// Execute a SQL query and prints output
    Query {
        /// A SQL command literal, wrapped in quotes. Ignored if '-f' is specified.
        sql: Option<String>,

        /// Run the contents of the provided .sql file 
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Use Rust's Debug output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// TODO
    Game
}

fn process_query(conn: &Connection, sql_string: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut query = conn.prepare(&sql_string)?;
    let num_columns = query.column_count();
    let cell_width = (120 / num_columns).min(16);
    
    for i in 0..num_columns {
        print!("{:<1$}", query.column_name(i)?, cell_width);
    }
    print!("\n");
    
    let mut response = query.query([])?;

    while let Ok(Some(row)) = response.next() {
        for i in 0..num_columns {
            let value = row.get_ref(i)?;

            // TODO: make a pretty formatter for `value`.
            // https://docs.rs/rusqlite/latest/rusqlite/types/enum.ValueRef.html

            print!("{:<1$}", value, cell_width);
        }
        print!("\n");
    }

    Ok(())
}

fn process_query_v(conn: &Connection, sql_string: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut query = conn.prepare(&sql_string)?;
    let mut response = query.query(())?;
    while let Ok(Some(row)) = response.next() {
        println!("{:?}",row);
    }
    Ok(())
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
            println!("{} row(s) modified",conn.execute(&sql_string, ())?);
        } else if sql.is_some() {
            println!("{} row(s) modified",conn.execute(&sql.unwrap(), ())?);
        }
    } else if let Cli {command: Some(
        Commands::Query { 
            sql, 
            file, 
            verbose}
        )} = cli {
        if file.is_some() {
            let sql_string = std::fs::read_to_string(file.unwrap())?;
            if verbose { 
                process_query_v(&conn, sql_string)?;
            } else {
                process_query(&conn, sql_string)?;
            }
        } else if sql.is_some() {
            if verbose {
                process_query_v(&conn, sql.unwrap())?;
            } else {
                process_query(&conn, sql.unwrap())?;
            }
        }
    } else {
        println!("Type `sqll help`");
    }
    Ok(())
}
