use crate::result::Result;
use chrono::{Local, DateTime, Duration, TimeZone, Date, Datelike};
use crate::result::Error;
use std::fmt::Formatter;
use std::ops::Sub;

pub struct Remindable {
    pub key: String,
    pub name: String,
    pub last_update: Date<Local>,
    pub remind_interval: Duration,
}

impl Remindable {
    pub fn from_strings(key: String, name: String, last_update: String, remind_interval: String) -> Result<Self> {
        Ok(Remindable {
            key,
            name,
            last_update: Self::date_from_string(last_update)?,
            remind_interval: Self::duration_from_string(remind_interval)?,
        })
    }

    /// A string date looks like 1.1.2021, day month year separated by dots.
    fn date_from_string(value: String) -> Result<Date<Local>> {
        let split = value.split(".").collect::<Vec<_>>();

        Ok(Local.ymd(
            split[2].parse::<i32>()?,
            split[1].parse::<u32>()?,
            split[0].parse::<u32>()?,
        ))
    }

    /// A duration string looks like "W42".
    /// The first letter indicates the type of duration. Currently supported are weeks (w) and days (d).
    /// The remaining part indicates the amount.
    fn duration_from_string(value: String) -> Result<Duration> {
        let type_part = &value[..1];
        let amount_part = &value[1..];
        let amount = amount_part.parse::<i64>()?;

        Ok(match type_part {
            "d" | "D" => Duration::days(amount),
            "w" | "W" => Duration::weeks(amount),
            _ => return Err(Error::RemindableValueParse(type_part.to_string()))
        })
    }

    pub fn is_todo(&self) -> bool {
        Local::today().sub(self.remind_interval) > self.last_update
    }

    pub fn set_done_today(&mut self) {
        self.last_update = Local::today()
    }

    pub fn get_last_update_string(&self) -> String {
        format!("{}.{}.{}", self.last_update.day(), self.last_update.month(), self.last_update.year())
    }

    pub fn get_remind_interval_string(&self) -> String {
        match self.remind_interval.num_weeks() {
            n if n > 1 => format!("W{}", n),
            _ => format!("D{}", self.remind_interval.num_days())
        }
    }
}

impl std::fmt::Display for Remindable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\" {} {}",
               self.key,
               self.name,
               self.get_last_update_string(),
               self.get_remind_interval_string())
    }
}