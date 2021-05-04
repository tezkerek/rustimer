use chrono::{DateTime, Duration, TimeZone};
use std::fmt::Display;

pub trait Pretty {
    fn pretty(&self) -> String;
}

impl Pretty for Duration {
    fn pretty(&self) -> String {
        let days = self.num_days();
        let hours = self.num_hours() % 24;
        let minutes = self.num_minutes() % 60;
        let seconds = self.num_seconds() % 60;
        match days {
            0 => format!("{:02}h {:02}min {:02}s", hours, minutes, seconds),
            _ => format!(
                "{}d {:02}h {:02}min {:02}s",
                days, hours, minutes, seconds
            ),
        }
    }
}

impl<T: TimeZone> Pretty for DateTime<T>
where
    T::Offset: Display,
{
    fn pretty(&self) -> String {
        self.format("%F %T").to_string()
    }
}
