use chrono::prelude::*;

#[derive(Debug)]
pub enum DateTimeError {
    InvalidDate,
    InvalidTime,
    InvalidDateTime,
}

pub type DateTimeResult = Result<String, DateTimeError>;

pub struct DateTimeFormatter;

impl DateTimeFormatter {
    pub fn fmt_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTimeResult {
        match Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .single()
        {
            Some(dt) => Ok(dt.format("%Y%m%dT%H%M%S").to_string()),
            None => Err(DateTimeError::InvalidDateTime),
        }
    }

    pub fn fmt_time(hour: u32, minute: u32, second: u32) -> DateTimeResult {
        match NaiveTime::from_hms_opt(hour, minute, second) {
            Some(t) => Ok(t.format("%H%M%S").to_string()),
            None => Err(DateTimeError::InvalidTime),
        }
    }

    pub fn fmt_date(year: i32, month: u32, day: u32) -> DateTimeResult {
        let result = NaiveDate::from_ymd_opt(year, month, day);
        match result {
            Some(d) => Ok(d.format("%Y%m%d").to_string()),
            None => Err(DateTimeError::InvalidDate),
        }
    }
}
