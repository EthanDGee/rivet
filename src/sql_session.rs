use sqlite::Connection;

pub struct SqlSession {
    sql_path: String,
    connection: Connection,
}

impl SqlSession {
    pub fn new(sql_path: String) -> Result<Self, sqlite::Error> {
        let connection = Connection::open(&sql_path)?;
        Ok(SqlSession {
            sql_path,
            connection,
        })
    }
}
