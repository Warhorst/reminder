use clap::Clap;

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
        Add(params) => Database::open()?.add_remindable(Remindable::try_from(params)?),
        DoneToday(params) => Database::open()?.set_remindable_done_today(params.key),
        SetLastUpdate(params) => Database::open()?.update_last_update(params.key, params.new_last_update),
        SetRemindInterval(params) => Database::open()?.update_remindable_interval(params.key, params.new_remind_interval),
        Delete(params) => Database::open()?.delete_entry_by_key(params.key),
        GetAll => Ok(Database::open()?.get_remindables()?.iter().for_each(|rem| println!("{}", rem))),
        Todos => Ok(Database::open()?.get_remindables()?.iter().filter(|rem| rem.is_todo()).for_each(|rem| println!("{}", rem))),
    }
}

#[derive(Clap)]
enum Command {
    Add(AddParams),
    DoneToday(DoneTodayParams),
    SetLastUpdate(SetLastUpdateParams),
    SetRemindInterval(SetRemindIntervalParams),
    Delete(DeleteParams),
    /// Return all remindables from the database and print their data on the console.
    GetAll,
    /// Return all remindables which need to be done.
    Todos,
}

/// Add a remindable to the database.
#[derive(Clap)]
struct AddParams {
    /// Key of the remindable. Used to access the remindable later.
    pub key: String,
    /// Name/description of this remindable. Only used for easier identification.
    pub name: String,
    /// Date of the last update. The format is "DD.MM.YYYY". Day and month don't need to be two digits.
    pub last_update: String,
    /// Maximum duration after which this remindable must be done again. Possible inputs are "D<n>" for n days ore "W<n>" for n weeks.
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

/// Set the date where a specific remindable was done to today.
#[derive(Clap)]
struct DoneTodayParams {
    /// Key which identifies the remindable to be updated.
    pub key: String
}

/// Set the last update for a specific remindable to the given value.
#[derive(Clap)]
struct SetLastUpdateParams {
    /// Key which identifies the remindable to be updated.
    pub key: String,
    /// New date of the last update. The format is "DD.MM.YYYY". Day and month don't need to be two digits.
    pub new_last_update: String
}

/// Set the remind interval for a specific remindable to the given value.
#[derive(Clap)]
struct SetRemindIntervalParams {
    /// Key which identifies the remindable to be updated.
    pub key: String,
    /// New maximum duration after which this remindable must be done again. Possible inputs are "D<n>" for n days ore "W<n>" for n weeks.
    pub new_remind_interval: String
}

/// Delete a remindable. This cannot be undone.
#[derive(Clap)]
struct DeleteParams {
    /// Key which identifies the remindable to be deleted.
    pub key: String
}