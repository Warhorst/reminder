use clap::Clap;
use rusqlite::Connection;

use crate::result::Error;
use crate::result::Result;
use crate::remindable::Remindable;
use std::fmt::Formatter;
use rusqlite::types::Type::Null;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open() -> Result<Self> {
        let connection = Connection::open("reminder.db")?;

        connection.execute("\
        CREATE TABLE IF NOT EXISTS Remindables (
            name TEXT PRIMARY KEY,
            last_update TEXT,
            remind_interval TEXT,
            previous_update TEXT
        );", [])?;

        Ok(Database { connection })
    }

    pub fn get_remindables(&self) -> Result<Vec<Remindable>> {
        let mut statement = self.connection.prepare("\
            SELECT r.name, r.last_update, r.remind_interval, r.previous_update FROM Remindables r;
        ")?;

        let result = statement.query_map([], |row| {
            Ok(Remindable::from_strings(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                Some(row.get(3)?),
            )?)
        })?;
        Ok(result.map(|r| r.unwrap()).collect())
    }

    pub fn add_entry(&self, name: String, last_update: String, remind_interval: String, previous_update: Option<String>) -> Result<()> {
        Remindable::from_strings(name.clone(), last_update.clone(), remind_interval.clone(), previous_update.clone())?;

        self.connection.execute("\
            INSERT INTO Remindables (name, last_update, remind_interval, previous_update)
            VALUES (?1, ?2, ?3, ?4);
        ", &[&name, &last_update, &remind_interval, &previous_update.unwrap_or_default()])?;

        Ok(())
    }

    pub fn delete_entry_by_name(&self, name: String) -> Result<()> {
        self.connection.execute("\
            DELETE FROM Remindables
            WHERE name LIKE ?1
        ", &[&name])?;
        Ok(())
    }
}