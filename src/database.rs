use rusqlite::{Connection, Row};

use crate::result::Error;
use crate::result::Result;
use crate::remindable::Remindable;
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
        let mut remindable = self.get_remindable_by_key(key)?;
        remindable.set_done_today();
        self.update_remindable(remindable)?;
        Ok(())
    }

    pub fn update_last_update(&self, key: String, new_last_update: String) -> Result<()> {
        let mut remindable = self.get_remindable_by_key(key)?;
        remindable.set_last_update(new_last_update)?;
        self.update_remindable(remindable)
    }

    pub fn update_remindable_interval(&self, key: String, new_remind_interval: String) -> Result<()> {
        let mut remindable = self.get_remindable_by_key(key)?;
        remindable.set_remind_interval(new_remind_interval)?;
        self.update_remindable(remindable)
    }

    pub fn get_remindable_by_key(&self, key: String) -> Result<Remindable> {
        Ok(self.connection
            .prepare("SELECT r.key, r.name, r.last_update, r.remind_interval FROM Remindables r WHERE key LIKE ?1")?
            .query_row(&[&key], |row| Ok(Remindable::try_from(row)?))?)
    }

    pub fn update_remindable(&self, remindable: Remindable) -> Result<()> {
        self.connection.execute("\
            UPDATE Remindables
            SET name = ?1, last_update = ?2, remind_interval = ?3
            WHERE key LIKE ?4
        ", &[&remindable.name, &remindable.get_last_update_string(), &remindable.get_remind_interval_string(), &remindable.key])?;

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