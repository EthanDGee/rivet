use color_eyre::eyre::{Result, eyre};
use rusqlite::{Connection, types::ValueRef};

pub struct SqlSession {
    sql_path: String,
    connection: Connection,
    pub read_only: bool,
}



impl SqlSession {
    pub fn extract_column_names(&self, query: &str) -> Result<Vec<String>> {
        if query.is_empty() {
            return Err(eyre!("Empty Query provided to extract_column_names"));
        }

        let statement = self.connection.prepare(query)?;
        let column_names: Vec<String> = statement
            .column_names()
            .iter()
            .map(|name| name.to_string())
            .collect();

        Ok(column_names)
    }

    pub fn new(sql_path: String, read_only: bool) -> Self {
        // attempt to connect to database
        let connection = match Connection::open(&sql_path) {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("SqlSession error: {}", e);
                std::process::exit(1);
            }
        };
        //check to see if the database is read_only
        let read_only_db: bool = match connection.is_readonly("main") {
            Ok(read_only_db) => read_only_db,
            Err(e) => {
                eprintln!("Failed to determine if {} is read_only", e);
                std::process::exit(1)
            }
        };
        // if it is a read only database and user specifies write operations throw an error
        if read_only_db & !read_only {
            eprint!(
                "Unable to open {0} in write mode. {0} is a read only database.",
                sql_path
            );
            std::process::exit(1);
        }

        SqlSession {
            sql_path,
            connection,
            read_only,
        }
    }

    pub fn select(&self, query: &str) -> Result<Vec<Vec<String>>> {
        if query.is_empty() {
            return Err(eyre!("Empty Query"));
        }

        let mut statement = match self.connection.prepare(&query) {
            Ok(statement) => statement,
            Err(e) => {
                return Err(eyre!("SELECT query could not be executed\n{}", e));
            }
        };

        let column_count = statement.column_count();
        let mut rows = statement.query([])?;
        let mut result_rows: Vec<Vec<String>> = Vec::new();

        while let Some(row) = rows.next()? {
            let mut result_row: Vec<String> = Vec::new();
            for i in 0..column_count {
                let value = row.get_ref(i)?;
                result_row.push(match value {
                    ValueRef::Null => "NULL".to_string(),
                    ValueRef::Integer(i) => i.to_string(),
                    ValueRef::Real(f) => f.to_string(),
                    ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    ValueRef::Blob(b) => String::from_utf8_lossy(b).to_string(),
                });
            }
            result_rows.push(result_row);
        }

        Ok(result_rows)
    }

    pub fn execute(&self, query: &str) -> Result<usize> {
        if query.is_empty() {
            return Err(eyre!("Empty Query"));
        }

        // check if it's read only check for write operations and exit early
        if self.read_only {
            return Err(eyre!(
                "Attempted an {} operation on a read only database",
                query.trim_start().to_uppercase().as_str()
            ));
        }

        let changes = self.connection.execute(query, [])?;
        Ok(changes)
    }

    pub fn get_change_count(&self) -> usize {
        self.connection.changes() as usize
    }

    pub fn commit(&self) {
        let _ = self.connection.execute("COMMIT", []);
    }
}
