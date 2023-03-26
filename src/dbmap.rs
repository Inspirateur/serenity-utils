use std::marker::PhantomData;
use rusqlite::Connection;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DBMap<K: ToString + From<String>, V: ToString + From<String>> {
    path: String,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>
}

impl<K: ToString + From<String>, V: ToString + From<String>> DBMap<K, V> {
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
            key_type: PhantomData,
            value_type: PhantomData
        })
    }

    pub fn insert(&self, key: K, value: V) -> Result<()> {
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

    pub fn get(&self, key: K) -> Result<V> {
        let conn = Connection::open(&self.path)?;
        let key = key.to_string();
        let mut stmt = conn
            .prepare("SELECT value FROM Map WHERE key = ?1")
            .unwrap();
        let value = stmt
            .query_row([key], |row| {
                row.get::<usize, String>(0)
            })?;
        Ok(value.into())
    }

    pub fn get_keys(&self, value: V) -> Result<Vec<K>> {
        let conn = Connection::open(&self.path)?;
        let value = value.to_string();
        let mut stmt = conn
            .prepare("SELECT key FROM Map WHERE value = ?1")
            .unwrap();
        let values = stmt
            .query_map([value], |row| {
                row.get::<usize, String>(0)
            })?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        Ok(values.into_iter().map(String::into).collect())
    }
}