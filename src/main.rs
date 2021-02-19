use chrono::Duration;
use clap::{App, Arg, ArgMatches, SubCommand};

mod task;

use task::Task;

fn fmt_duration(duration: Duration) -> String {
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn prompt(duration: Duration) -> String {
    return format!("[{}]", fmt_duration(duration));
}

fn get_arg_matches() -> ArgMatches<'static> {
    return App::new("Rustimer")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("start").arg(
                Arg::with_name("name")
                    .help("The name of the task")
                    .required(true),
            ),
        )
        .get_matches();
}

fn main() {
    let arg_matches = get_arg_matches();

    // Default action is to show status
    if arg_matches.subcommand_name() == None {
        print_status();
    }

    match arg_matches.subcommand() {
        ("start", Some(subargs)) => {
            let task = Task::create_now(subargs.value_of("name").unwrap());
            println!("{:?}", task);
        }
        _ => {}
    }
}

fn print_status() {
    eprintln!("Working on {}", "nothing");
}
