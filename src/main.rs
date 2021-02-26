use std::{error::Error, path::Path};

use chrono::Duration;
use clap::{App, Arg, ArgMatches, SubCommand};

mod task;
use task::{Task, TaskStore};

trait PrettyTime {
    fn pretty(&self) -> String;
}

impl PrettyTime for Duration {
    fn pretty(&self) -> String {
        let hours = self.num_hours() % 24;
        let minutes = self.num_minutes() % 60;
        let seconds = self.num_seconds() % 60;
        return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    }
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

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = get_arg_matches();

    let mut store = TaskStore::from_file(Path::new("store.json"))?;

    // Default action is to show status
    if arg_matches.subcommand_name() == None {
        print_status(&store);
    }

    match arg_matches.subcommand() {
        ("start", Some(subargs)) => {
            let task = Task::create_now(subargs.value_of("name").unwrap());
            let new_task = store.add(task);
            eprintln!("New task: {}", new_task.name);
        }
        _ => {}
    }
    store.save()?;

    Ok(())
}

fn print_status(store: &TaskStore) {
    let active_tasks = store.active_tasks();

    if active_tasks.is_empty() {
        eprintln!("No active tasks");
    } else {
        eprintln!("Working on:");
        for task in active_tasks {
            eprintln!("{} | Elapsed: {}", task.name, task.elapsed().pretty());
        }
    }
}
