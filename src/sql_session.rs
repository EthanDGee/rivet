use core::prelude;
use std::f32::consts::E;

use color_eyre::eyre::{Result, eyre};
use sqlite::{Connection, Value};
pub struct SqlSession {
    sql_path: String,
    connection: Connection,
    read_only: bool,
}

impl SqlSession {
    pub fn new(sql_path: String) -> Self {
        // attempt to connect to database
        let connection = match Connection::open(&sql_path) {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("SqlSession{}", e);
                std::process::exit(1);
            }
        };

        let read_only: bool = true;

        SqlSession {
            sql_path,
            connection,
            read_only,
        }
    }

    pub fn select(&self, query: String) -> Result<Vec<Vec<String>>> {
        if query.is_empty() {
            return Err(eyre!("Empty Query"));
        }

        let mut statement = match self.connection.prepare(query) {
            Ok(statement) => statement,
            Err(e) => {
                return Err(eyre!("SELECT query could not be executed\n{}", e));
            }
        };

        let mut rows: Vec<Vec<String>> = Vec::new();
        let column_count = statement.column_count();

        loop {
            match statement.next() {
                Ok(sqlite::State::Row) => {
                    let row: Result<Vec<String>, sqlite::Error> =
                        (0..column_count).map(|i| statement.read(i)).collect();
                    let row = row?; // Propagate error if any column read fails
                    rows.push(row);
                }
                Ok(sqlite::State::Done) => break,
                Err(e) => return Err(e.into()), // Propagate error
            }
        }

        Ok(rows)
    }

    pub fn execute(&self, query: String) -> Result<usize> {
        if query.is_empty() {
            return Err(eyre!("Empty Query"));
        }

        // check if it's read only check for write operations and exit early
        if self.read_only {
            return Err(eyre!(
                "Attempted an {} operation on a read only database",
                query.trim_start().to_uppercase()
            ));
        }

        self.connection.execute(query)?;
        Ok(self.connection.change_count())
    }
    pub fn get_change_count(&self) -> usize {
        self.connection.change_count()
    }

    pub fn commit(&self) {
        let _ = self.connection.execute("COMMIT");
    }
}
