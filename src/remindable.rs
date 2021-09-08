use crate::result::Result;
use chrono::{Local, DateTime, Duration, TimeZone, Date};
use crate::result::Error;
use std::fmt::Formatter;
use std::ops::Sub;

pub struct Remindable {
    pub name: String,
    pub last_update: Date<Local>,
    pub remind_interval: Duration,
    pub previous_update: Option<Date<Local>>
}

impl Remindable {
    pub fn from_strings(name: String, last_update: String, remind_interval: String, previous_update: Option<String>) -> Result<Self> {
        Ok(Remindable {
            name,
            last_update: Self::parse_date(last_update)?,
            remind_interval: Self::parse_duration(remind_interval)?,
            previous_update: match previous_update {
                Some(val) if !val.is_empty() => Some(Self::parse_date(val)?),
                _ => None
            }


        })
    }

    /// A string date looks like 1.1.2021, day month year separated by dots.
    fn parse_date(value: String) -> Result<Date<Local>> {
        let split = value.split(".").collect::<Vec<_>>();

        Ok(Local.ymd(
            split[2].parse::<i32>()?,
            split[1].parse::<u32>()?,
            split[0].parse::<u32>()?
        ))
    }

    /// A duration string looks like "W42".
    /// The first letter indicates the type of duration. Currently supported are weeks (w) and days (d).
    /// The remaining part indicates the amount.
    fn parse_duration(value: String) -> Result<Duration> {
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
}

impl std::fmt::Display for Remindable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}",
               self.name,
               self.last_update,
               self.remind_interval,
               match self.previous_update {
                   Some(val) => val.to_string(),
                   None => "".to_string()
               })
    }
}