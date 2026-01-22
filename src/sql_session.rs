use color_eyre::eyre::{Result, eyre};
use sqlite::Connection;
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

    pub fn commit(&self) -> Result<usize> {
        self.connection.execute("COMMIT")?;
        Ok(self.connection.change_count())
    }
}
