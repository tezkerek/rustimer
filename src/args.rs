use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, NaiveTime, TimeZone};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const LIST: &str = "list";
pub mod list {
    pub const KIND: &str = "kind";
    pub const KIND_ALL: &str = "all";
    pub const KIND_RUNNING: &str = "running";
    pub const KIND_COMPLETED: &str = "completed";
    pub const KIND_VALUES: [&str; 3] = [KIND_ALL, KIND_RUNNING, KIND_COMPLETED];
    pub const KIND_DEFAULT: &str = "all";
}

pub const START: &str = "start";
pub mod start {
    pub const NAME: &str = "name";
    pub const TAGS: &str = "tags";
    pub const START_TIME: &str = "start_time";
}

pub const COMPLETE: &str = "complete";
pub mod complete {
    pub const ID: &str = "id";
    pub const END_TIME: &str = "end_time";
}

pub const DELETE: &str = "delete";
pub mod delete {
    pub const ID: &str = "id";
}

pub fn get_arg_matches<'a>() -> ArgMatches<'a> {
    App::new("Rustimer")
        .version("0.1")
        .subcommand(
            SubCommand::with_name(LIST).arg(
                Arg::with_name(list::KIND)
                    .help("What kind of tasks to list (default: all)")
                    .possible_values(&list::KIND_VALUES)
                    .default_value(list::KIND_DEFAULT)
                    .required(false),
            ),
        )
        .subcommand(
            SubCommand::with_name(START)
                .arg(
                    Arg::with_name(start::NAME)
                        .help("Name of the task")
                        .required(true),
                )
                .arg(
                    Arg::with_name(start::START_TIME)
                        .help("When to start the task")
                        .long("at")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name(start::TAGS)
                    .help("Space-separated tags for the task")
                    .multiple(true)
                    .takes_value(true)
                    .required(false)
                ),
        )
        .subcommand(
            SubCommand::with_name(COMPLETE)
                .arg(
                    Arg::with_name(complete::ID)
                        .help("ID of the task")
                        .required(true),
                )
                .arg(
                    Arg::with_name(complete::END_TIME)
                        .help("When to complete the task")
                        .long("at")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(DELETE).arg(
                Arg::with_name(delete::ID)
                    .help("ID of the task")
                    .required(true),
            ),
        )
        .get_matches()
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
