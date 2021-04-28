mod args;
mod pretty;
mod task;

use anyhow::{anyhow, Result};
use clap::ArgMatches;
use prettytable::{cell, Row, Table};
use std::{borrow::Borrow, path::Path};

use pretty::Pretty;
use task::{Task, TaskStore};

fn main() -> Result<()> {
    let arg_matches = args::get_arg_matches();

    // Default action is to show status
    if arg_matches.subcommand_name() == None {
        print_status(&get_store()?);
    }

    match arg_matches.subcommand() {
        (args::LIST, Some(subargs)) => {
            handle_list(subargs)?;
        }
        (args::START, Some(subargs)) => {
            handle_start(subargs)?;
        }
        (args::COMPLETE, Some(subargs)) => {
            handle_complete(subargs)?;
        }
        _ => {}
    }

    Ok(())
}

fn get_store() -> Result<TaskStore> {
    TaskStore::from_file(Path::new("store.json"))
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

fn handle_list(args: &ArgMatches) -> Result<()> {
    let store = get_store()?;

    match args.value_of(args::list::KIND).unwrap() {
        args::list::KIND_ALL => print_tasks(&store.all()),
        args::list::KIND_COMPLETED => print_tasks(&store.completed_tasks()),
        _ => {}
    }

    Ok(())
}

fn handle_start(args: &ArgMatches) -> Result<()> {
    let mut store = get_store()?;
    let new_task =
        store.add(Task::create_now(args.value_of(args::start::NAME).unwrap()));
    eprintln!("New task: {}", new_task.name);
    store.save()?;
    Ok(())
}

fn handle_complete(args: &ArgMatches) -> Result<()> {
    let mut store = get_store()?;
    let id: u32 = args
        .value_of(args::complete::NAME)
        .unwrap()
        .parse()
        .unwrap();
    let task: &mut Task = store
        .get_mut(id)
        .ok_or(anyhow!(format!("Task {} does not exist", id)))?;
    task.complete_now();
    eprintln!("Completed task {}", task.name);
    store.save()?;
    Ok(())
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
