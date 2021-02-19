use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{fs::File, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl Task {
    pub fn create_now(name: &str) -> Self {
        Self::new(name, Utc::now(), None)
    }

    pub fn new(
        name: &str,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
    ) -> Self {
        Task {
            name: String::from(name),
            start_time,
            end_time,
        }
    }

    pub fn is_completed(&self) -> bool {
        match self.end_time {
            Some(_) => true,
            None => false,
        }
    }

    pub fn save(&self) {}
}

#[derive(Serialize, Deserialize)]
pub struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    pub fn from_file(path: Path) -> Result<Self, Err> {
        let file = File::open("store.json")?;
    }

    pub fn save(task: &Task) {}

    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
