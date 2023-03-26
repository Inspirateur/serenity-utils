use rusqlite::{Connection};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DBMap {
    path: String,
}

impl DBMap {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Map (
                key TEXT PRIMARY KEY,
                value TEXT
            )",
            [],
        )?;
        Ok(DBMap {
            path: db_path.to_string(),
        })
    }

    pub fn insert<S1, S2>(&self, key: S1, value: S2) -> Result<()> 
    where S1: ToString, S2: ToString {
        let conn = Connection::open(&self.path)?;
        let key = key.to_string();
        let value = value.to_string();
        conn.execute(
            "INSERT OR REPLACE INTO Map (key, value)
            VALUES (?1, ?2)", 
            [key, value]
        )?;
        Ok(())
    }

    pub fn get<S: ToString>(&self, key: S) -> Result<String> {
        let conn = Connection::open(&self.path)?;
        let key = key.to_string();
        let mut stmt = conn
            .prepare("SELECT value FROM Map WHERE key = ?1")
            .unwrap();
        let value = stmt
            .query_row([key], |row| {
                row.get::<usize, String>(0)
            })?;
        Ok(value)
    }
}