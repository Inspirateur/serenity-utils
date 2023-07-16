use std::marker::PhantomData;
use rusqlite::{Connection, params, ToSql, types::FromSql};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DBMap<K, V> {
    path: String,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>
}

impl<K, V> DBMap<K, V> {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Map (
                key BLOB PRIMARY KEY,
                value BLOB
            )",
            [],
        )?;
        Ok(DBMap {
            path: db_path.to_string(),
            key_type: PhantomData,
            value_type: PhantomData
        })
    }
}

impl<K: ToSql, V: ToSql> DBMap<K, V> {
    pub fn insert(&self, key: K, value: V) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "INSERT OR REPLACE INTO Map (key, value)
            VALUES (?1, ?2)", 
            params![key, value]
        )?;
        Ok(())
    }
}

impl<K: ToSql, V: FromSql> DBMap<K, V> {
    pub fn get(&self, key: K) -> Result<V> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn
            .prepare("SELECT value FROM Map WHERE key = ?1")
            .unwrap();
        let value = stmt
            .query_row([key], |row| {
                row.get::<usize, V>(0)
            })?;
        Ok(value)
    }
}

impl<K: FromSql, V: ToSql> DBMap<K, V> {
    pub fn get_keys(&self, value: V) -> Result<Vec<K>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn
            .prepare("SELECT key FROM Map WHERE value = ?1")
            .unwrap();
        let values = stmt
            .query_map([value], |row| {
                row.get::<usize, K>(0)
            })?
            .collect::<Result<Vec<K>, rusqlite::Error>>()?;
        Ok(values)
    }
}