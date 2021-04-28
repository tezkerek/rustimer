use std::fmt::Display;
use chrono::{DateTime, Duration, TimeZone};

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
            0 => format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
            1 => format!("1 day, {:02}:{:02}:{:02}", hours, minutes, seconds),
            _ => format!(
                "{} days, {:02}:{:02}:{:02}",
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


