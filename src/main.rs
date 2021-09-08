use chrono::{Date, Local, TimeZone};
use clap::Clap;
use rusqlite::Connection;

use Command::*;
use result::Result;

use crate::database::{Database};
use crate::remindable::Remindable;

mod database;
mod remindable;
pub mod result;

fn main() -> Result<()> {
    let command = Command::parse();

    match command {
        GetAll => Ok(Database::open()?.get_remindables()?.iter().for_each(|e| println!("{}", e))),
        Todos => Ok(Database::open()?.get_remindables()?.iter().filter(|rem| rem.is_todo()).for_each(|rem| println!("{}", rem))),
        Add(params) => Database::open()?.add_entry(params.name, params.last_update, params.remind_interval, params.previous_update),
        Delete(params) => Database::open()?.delete_entry_by_name(params.name)
    }
}

#[derive(Clap)]
enum Command {
    GetAll,
    Todos,
    Add(AddParams),
    Delete(DeleteParams)
}

#[derive(Clap)]
struct AddParams {
    pub name: String,
    pub last_update: String,
    pub remind_interval: String,
    pub previous_update: Option<String>,
}

#[derive(Clap)]
struct DeleteParams {
    pub name: String
}