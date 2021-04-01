mod args;
mod task;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Duration, TimeZone};
use clap::ArgMatches;
use prettytable::{cell, Row, Table};
use std::path::Path;
use std::{borrow::Borrow, fmt::Display};
use task::{Task, TaskStore};

trait Pretty {
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
        (args::LIST, Some(subargs)) => {
            match subargs.value_of(args::list::KIND).unwrap() {
                args::list::KIND_ALL => print_tasks(&store.all()),
                args::list::KIND_COMPLETED => {
                    print_tasks(&store.completed_tasks())
                }
                _ => {}
            }
        }
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
        print_tasks(active_tasks.as_slice());
    }
}

fn print_tasks<T: Borrow<Task>>(tasks: &[(u32, T)]) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        cell!("ID"),
        cell!("Name"),
        cell!("Interval"),
        cell!("Elapsed"),
    ]));

    for (id, btask) in tasks {
        let task: &Task = btask.borrow();
        let interval_str = format!(
            "{} - {}",
            task.start_time.pretty(),
            task.end_time.map(|d| d.pretty()).unwrap_or(String::new())
        );

        let row = Row::new(vec![
            cell!(r->id.to_string().as_str()),
            cell!(task.name.as_str()),
            cell!(&interval_str),
            cell!(r->task.elapsed().pretty().as_str()),
        ]);
        table.add_row(row);
    }
    table.printstd();
}

// #[derive(Debug)]
// enum Entity {
//     Task,
//     Project,
// }

// #[derive(Debug)]
// enum Error {
//     Store(task::Error),
//     NotFound(Entity),
// }

// impl From<task::Error> for Error {
//     fn from(err: task::Error) -> Self {
//         Self::Store(err)
//     }
// }
