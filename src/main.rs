mod args;
mod pretty;
mod task;

use anyhow::{Context, Result};
use clap::Parser;
use prettytable::{cell, Row, Table};
use std::{borrow::Borrow, path::Path};

use args::{Cli, CompleteArgs, DeleteArgs, ListArgs, StartArgs};
use pretty::Pretty;
use task::{Task, TaskStore};

fn main() -> Result<()> {
    let args = Cli::parse();

    // Default action is to show status
    if let Some(command) = args.command {
        match command {
            args::Command::List(subargs) => {
                handle_list(&subargs)?;
            }
            args::Command::Start(subargs) => {
                handle_start(&subargs)?;
            }
            args::Command::Complete(subargs) => {
                handle_complete(&subargs)?;
            }
            args::Command::Delete(subargs) => {
                handle_delete(&subargs)?;
            }
        }
    } else {
        print_status(&get_store()?);
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
            cell!(task.tags.join(",")),
        ]);
        table.add_row(row);
    }
    table.printstd();
}

fn handle_list(args: &ListArgs) -> Result<()> {
    let store = get_store()?;

    match &args.kind {
        args::ListKind::All => print_tasks(&store.all()),
        args::ListKind::Running => print_tasks(&store.running_tasks()),
        args::ListKind::Completed => print_tasks(&store.completed_tasks()),
    }

    Ok(())
}

fn handle_start(args: &StartArgs) -> Result<()> {
    let mut store = get_store()?;

    let tags: Vec<String> = args.tags.split(',').map(String::from).collect();

    let new_task = store.add(if let Some(start_time) = args.start_time {
        Task::new(&args.name, &tags, start_time, None)
    } else {
        Task::create_now(&args.name, &tags)
    });
    eprintln!("New task: {}", new_task.name);
    store.save()
}

fn handle_complete(args: &CompleteArgs) -> Result<()> {
    let mut store = get_store()?;
    let task: &mut Task = store
        .get_mut(args.id)
        .with_context(|| format!("Task with ID {} does not exist", args.id))?;

    if let Some(end_time) = args.end_time {
        task.complete_at(end_time);
    } else {
        task.complete_now();
    }
    eprintln!("Completed task \"{}\"", task.name);
    store.save()
}

fn handle_delete(args: &DeleteArgs) -> Result<()> {
    let mut store = get_store()?;

    let removed_task = store
        .remove(&args.id)
        .with_context(|| format!("Task with ID {} does not exist", args.id))?;
    eprintln!("Task \"{}\" has been deleted", removed_task.name);

    store.save()
}
