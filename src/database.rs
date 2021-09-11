use clap::Clap;
use rusqlite::{Connection, Row};

use crate::result::Error;
use crate::result::Result;
use crate::remindable::Remindable;
use std::fmt::Formatter;
use rusqlite::types::Type::Null;
use std::convert::TryFrom;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open() -> Result<Self> {
        let connection = Connection::open("reminder.db")?;

        connection.execute("\
        CREATE TABLE IF NOT EXISTS Remindables (
            key TEXT PRIMARY KEY,
            name TEXT,
            last_update TEXT,
            remind_interval TEXT
        );", [])?;

        Ok(Database { connection })
    }

    pub fn get_remindables(&self) -> Result<Vec<Remindable>> {
        let mut statement = self.connection.prepare("\
            SELECT key, name, last_update, remind_interval FROM Remindables;
        ")?;

        let result = statement.query_map([], |row| {
            Ok(Remindable::try_from(row)?)
        })?;
        Ok(result.map(|r| r.unwrap()).collect())
    }

    pub fn add_remindable(&self, remindable: Remindable) -> Result<()> {
        self.connection.execute("\
            INSERT INTO Remindables (key, name, last_update, remind_interval)
            VALUES (?1, ?2, ?3, ?4);
        ", &[&remindable.key, &remindable.name, &remindable.get_last_update_string(), &remindable.get_remind_interval_string()])?;

        Ok(())
    }

    pub fn delete_entry_by_key(&self, key: String) -> Result<()> {
        self.connection.execute("\
            DELETE FROM Remindables
            WHERE key LIKE ?1
        ", &[&key])?;
        Ok(())
    }

    pub fn set_remindable_done_today(&self, key: String) -> Result<()> {
        let mut remindable = self.connection
            .prepare("SELECT r.key, r.name, r.last_update, r.remind_interval FROM Remindables r WHERE key LIKE ?1")?
            .query_row(&[&key], |row| Ok(Remindable::try_from(row)?))?;

        remindable.set_done_today();

        self.connection.execute("\
            UPDATE Remindables
            SET last_update = ?1
            WHERE key LIKE ?2
        ", &[&remindable.get_last_update_string(), &remindable.key])?;

        Ok(())
    }
}

impl TryFrom<&Row<'_>> for Remindable {
    type Error = Error;

    fn try_from(row: &Row) -> Result<Self> {
        Remindable::from_strings(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        )
    }
}