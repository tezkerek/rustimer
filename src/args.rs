use clap::{App, Arg, ArgMatches, SubCommand};

pub const ALL: &str = "all";

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

pub fn get_arg_matches() -> ArgMatches<'static> {
    App::new("Rustimer")
        .version("0.1")
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
