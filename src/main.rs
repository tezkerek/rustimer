mod args;
mod pretty;
mod task;

use anyhow::{Context, Result};
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
        (args::DELETE, Some(subargs)) => {
            handle_delete(subargs)?;
        }
        _ => {}
    }

    Ok(())
}

fn get_store() -> Result<TaskStore> {
    TaskStore::from_file(Path::new("store.json"))
}

fn print_status(store: &TaskStore) {
    let running_tasks = store.running_tasks();

    if running_tasks.is_empty() {
        eprintln!("No running tasks");
    } else {
        eprintln!("Working on:");
        print_tasks(running_tasks.as_slice());
    }
}

fn print_tasks<T: Borrow<Task>>(tasks: &[(u32, T)]) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        cell!("ID"),
        cell!("Name"),
        cell!("Interval"),
        cell!("Elapsed"),
        cell!("Tags"),
    ]));

    for (id, btask) in tasks {
        let task: &Task = btask.borrow();
        let interval_str = format!(
            "{} - {}",
            task.start_time.pretty(),
            task.end_time
                .map(|d| d.pretty())
                .unwrap_or("...".to_owned())
        );

        let row = Row::new(vec![
            cell!(r->id.to_string().as_str()),
            cell!(task.name.as_str()),
            cell!(&interval_str),
            cell!(r->task.elapsed().pretty().as_str()),
            cell!(r->task.tags.join(" ")),
        ]);
        table.add_row(row);
    }
    table.printstd();
}

fn handle_list(args: &ArgMatches) -> Result<()> {
    let store = get_store()?;

    match args.value_of(args::list::KIND).unwrap() {
        args::list::KIND_ALL => print_tasks(&store.all()),
        args::list::KIND_RUNNING => print_tasks(&store.running_tasks()),
        args::list::KIND_COMPLETED => print_tasks(&store.completed_tasks()),
        _ => {}
    }

    Ok(())
}

fn handle_start(args: &ArgMatches) -> Result<()> {
    let mut store = get_store()?;
    let name = args.value_of(args::start::NAME).unwrap();

    let tags: Vec<&str> = args
        .values_of(args::start::TAGS)
        .map(|vals| vals.collect())
        .unwrap_or(vec![]);

    let start_time = args
        .value_of(args::start::START_TIME)
        .map(|str| args::parse_local_datetime(str));

    let new_task = store.add(if let Some(start_time) = start_time {
        Task::new(name, &tags, start_time?, None)
    } else {
        Task::create_now(name, &tags)
    });
    eprintln!("New task: {}", new_task.name);
    store.save()
}

fn handle_complete(args: &ArgMatches) -> Result<()> {
    let mut store = get_store()?;
    let id: u32 = args.value_of(args::complete::ID).unwrap().parse()?;
    let task: &mut Task = store
        .get_mut(id)
        .with_context(|| format!("Task with ID {} does not exist", id))?;

    if let Some(end_time_res) = args
        .value_of(args::complete::END_TIME)
        .map(|str| args::parse_local_datetime(str))
    {
        task.complete_at(end_time_res?);
    } else {
        task.complete_now();
    }
    eprintln!("Completed task \"{}\"", task.name);
    store.save()
}

fn handle_delete(args: &ArgMatches) -> Result<()> {
    let mut store = get_store()?;
    let id: u32 = args.value_of(args::complete::ID).unwrap().parse()?;

    let removed_task = store
        .remove(&id)
        .with_context(|| format!("Task with ID {} does not exist", id))?;
    eprintln!("Task \"{}\" has been deleted", removed_task.name);

    store.save()
}
