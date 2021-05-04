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
    pub const START_TIME: &str = "start_time";
}

pub const COMPLETE: &str = "complete";
pub mod complete {
    pub const NAME: &str = "name";
    pub const END_TIME: &str = "end_time";
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
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(COMPLETE)
                .arg(
                    Arg::with_name(complete::NAME)
                        .help("Name of the task")
                        .required(true),
                )
                .arg(
                    Arg::with_name(complete::END_TIME)
                        .help("When to complete the task")
                        .long("at")
                        .required(false),
                ),
        )
        .get_matches()
}
