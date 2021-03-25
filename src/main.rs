mod args;
mod task;

use anyhow::{anyhow, Error};
use chrono::Duration;
use clap::ArgMatches;
use std::path::Path;
use task::{Task, TaskStore};

trait PrettyTime {
    fn pretty(&self) -> String;
}

impl PrettyTime for Duration {
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

fn main() -> Result<(), Error> {
    let arg_matches = args::get_arg_matches();
    handle_cmds(arg_matches)
}

fn handle_cmds(arg_matches: ArgMatches) -> Result<(), Error> {
    let mut store = TaskStore::from_file(Path::new("store.json"))?;

    // Default action is to show status
    if arg_matches.subcommand_name() == None {
        print_status(&store);
    }

    match arg_matches.subcommand() {
        (args::START, Some(subargs)) => {
            let task =
                Task::create_now(subargs.value_of(args::start::NAME).unwrap());
            let new_task = store.add(task);
            eprintln!("New task: {}", new_task.name);
        }
        (args::COMPLETE, Some(subargs)) => {
            let id: u32 = subargs
                .value_of(args::complete::NAME)
                .unwrap()
                .parse()
                .unwrap();
            let mut task = store
                .get_by_id(id)
                .ok_or(anyhow!(format!("Task {} does not exist", id)))?
                .clone();
            task.complete_now();
            let new_task = store.update(id, task);
            eprintln!("Completed task {}", new_task.name);
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
        for (id, task) in active_tasks {
            eprintln!(
                "{} | {} | Elapsed: {}",
                id,
                task.name,
                task.elapsed().pretty()
            );
        }
    }
}
