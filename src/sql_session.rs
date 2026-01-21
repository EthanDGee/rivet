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

    pub fn commit(&self) {
        self.connection.execute("COMMIT");
    }
}
