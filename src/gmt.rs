use crate::error;
use chrono::{
    DateTime,
    NaiveDateTime,
    Utc,
};

pub trait Gmt {
    fn to_gmt(&self) -> String;
    fn from_gmt<T: AsRef<str>>(date: T) -> Result<DateTime<Utc>, error::Error>;
}

impl Gmt for DateTime<Utc> {
    fn to_gmt(&self) -> String {
        format!("{}", self.format("%a, %d %b %Y %H:%M:%S GMT"))
    }

    fn from_gmt<T: AsRef<str>>(date: T) -> Result<DateTime<Utc>, error::Error> {
        Ok(DateTime::from_utc(
            NaiveDateTime::parse_from_str(date.as_ref(), "%a, %d %b %Y %H:%M:%S GMT")
                .map_err(error::Internal::from)?,
            Utc,
        ))
    }
}
