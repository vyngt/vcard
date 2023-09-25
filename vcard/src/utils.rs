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
    ) -> String {
        let dt = Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap();
        dt.format("%Y%m%dT%H%M%S").to_string()
    }

    pub fn fmt_time(hour: u32, minute: u32, second: u32) -> String {
        let dt = Utc
            .with_ymd_and_hms(2000, 4, 3, hour, minute, second)
            .unwrap();
        dt.format("%H%M%S").to_string()
    }

    pub fn fmt_date(year: i32, month: u32, day: u32) -> DateTimeResult {
        let result = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single();
        match result {
            Some(d) => Ok(d.format("%Y%m%d").to_string()),
            None => Err(DateTimeError::InvalidDate),
        }
    }
}
