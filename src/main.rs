use chrono::{Date, Local, TimeZone};
use clap::Clap;
use rusqlite::Connection;

use Command::*;
use result::Result;

use crate::database::{Database};
use crate::remindable::Remindable;
use crate::result::Error;
use std::convert::TryFrom;

mod database;
mod remindable;
pub mod result;

fn main() -> Result<()> {
    match Command::parse() {
        GetAll => Ok(Database::open()?.get_remindables()?.iter().for_each(|e| println!("{}", e))),
        Todos => Ok(Database::open()?.get_remindables()?.iter().filter(|rem| rem.is_todo()).for_each(|rem| println!("{}", rem))),
        Add(params) => Database::open()?.add_remindable(Remindable::try_from(params)?),
        Delete(params) => Database::open()?.delete_entry_by_key(params.name),
        DoneToday(params) => Database::open()?.set_remindable_done_today(params.key)
    }
}

#[derive(Clap)]
enum Command {
    GetAll,
    Todos,
    Add(AddParams),
    Delete(DeleteParams),
    DoneToday(DoneTodayParams)
}

#[derive(Clap)]
struct AddParams {
    pub key: String,
    pub name: String,
    pub last_update: String,
    pub remind_interval: String
}

impl TryFrom<AddParams> for Remindable {
    type Error = Error;

    fn try_from(params: AddParams) -> Result<Self> {
        Remindable::from_strings(
            params.key,
            params.name,
            params.last_update,
            params.remind_interval
        )
    }
}

#[derive(Clap)]
struct DeleteParams {
    pub name: String
}

#[derive(Clap)]
struct DoneTodayParams {
    pub key: String
}