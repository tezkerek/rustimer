use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, NaiveTime, TimeZone};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// List tasks
    List(ListArgs),
    /// Start a task
    Start(StartArgs),
    /// Complete a task
    Complete(CompleteArgs),
    /// Delete a task
    Delete(DeleteArgs),
}

#[derive(Clone, ValueEnum)]
pub enum ListKind {
    All,
    Running,
    Completed,
}

#[derive(Args)]
pub struct ListArgs {
    /// The kind of tasks to list
    #[arg(value_enum, default_value_t = ListKind::All)]
    pub kind: ListKind,
}

#[derive(Args)]
pub struct StartArgs {
    /// Name of the task
    pub name: String,

    /// Start time of the task
    #[arg(long = "at", value_parser = parse_local_datetime)]
    pub start_time: Option<DateTime<chrono::Local>>,

    /// Tags for the task
    pub tags: Vec<String>,
}

#[derive(Args)]
pub struct CompleteArgs {
    /// Task ID
    pub id: u32,

    /// Completion time for the task
    #[arg(value_parser = parse_local_datetime)]
    pub end_time: Option<DateTime<chrono::Local>>,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Task ID
    pub id: u32,
}

pub fn parse_local_datetime(s: &str) -> Result<DateTime<chrono::Local>> {
    let parsed: NaiveDateTime =
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| {
                // Today + hours, minutes, seconds
                NaiveTime::parse_from_str(s, "%H:%M:%S")
                    .or_else(|_| {
                        // Today + hours and minutes only
                        NaiveTime::parse_from_str(s, "%H:%M")
                    })
                    .map(|time| {
                        NaiveDateTime::new(
                            chrono::Local::today().naive_local(),
                            time,
                        )
                    })
            })
            .context("Failed to parse date")?;

    chrono::Local
        .from_local_datetime(&parsed)
        .earliest()
        .with_context(|| format!("Failed to parse date"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_parse_local_datetime() {
        let full = parse_local_datetime("2021-05-04 08:32:15").unwrap();
        assert_eq!(full, Local.ymd(2021, 5, 4).and_hms(8, 32, 15));

        let time = parse_local_datetime("08:32:15").unwrap();
        assert_eq!(time, Local::today().and_hms(8, 32, 15));

        assert_eq!(
            parse_local_datetime("8:03").unwrap(),
            Local::today().and_hms(8, 3, 0)
        );
    }
}
