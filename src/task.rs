use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap, fmt::Debug, fs::OpenOptions, io, io::BufReader,
    path::Path, path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub tags: Vec<String>,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
}

impl Task {
    pub fn create_now<S: Into<String>>(name: S, tags: &[String]) -> Self {
        Self::new(name.into(), tags, Local::now(), None)
    }

    pub fn new<S: Into<String>>(
        name: S,
        tags: &[String],
        start_time: DateTime<Local>,
        end_time: Option<DateTime<Local>>,
    ) -> Self {
        Task {
            name: name.into(),
            tags: tags.to_vec(),
            start_time,
            end_time,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.end_time != None
    }

    /// Get elapsed time until end if completed, or until now if ongoing
    pub fn elapsed(&self) -> Duration {
        match self.end_time {
            Some(end) => end - self.start_time,
            None => Local::now() - self.start_time,
        }
    }

    pub fn complete_at(&mut self, date: DateTime<Local>) {
        self.end_time = Some(date);
    }

    pub fn complete_now(&mut self) {
        self.complete_at(Local::now())
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskStore {
    tasks: BTreeMap<u32, Task>,

    #[serde(skip_serializing, default = "default_path")]
    save_path: PathBuf,
}

impl TaskStore {
    pub fn from_file(path: &Path) -> Result<Self> {
        let file_result = OpenOptions::new().read(true).open(path);

        let store = match file_result {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut store: TaskStore = serde_json::from_reader(reader)?;
                store.save_path = path.to_path_buf();
                store
            }
            Err(e) => match e.kind() {
                // Return an empty store if file is not found
                io::ErrorKind::NotFound => TaskStore {
                    tasks: BTreeMap::new(),
                    save_path: path.to_path_buf(),
                },
                _ => return Err(e.into()),
            },
        };

        Ok(store)
    }

    pub fn save(&self) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.save_path)?;

        serde_json::to_writer(file, self)
            .context("Failed to write changes to file")
    }

    pub fn add(&mut self, task: Task) -> &Task {
        // Find unclaimed id
        let new_id =
            self.tasks
                .keys()
                .fold(1, |acc, id| if *id == acc { acc + 1 } else { acc });
        self.tasks.insert(new_id, task);
        return self.tasks.get(&new_id).unwrap();
    }

    pub fn remove(&mut self, id: &u32) -> Option<Task> {
        self.tasks.remove(id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    pub fn filter<F: Fn(u32, &Task) -> bool>(&self, f: F) -> Vec<(u32, &Task)> {
        self.tasks
            .iter()
            .filter_map(|(&id, t)| if f(id, t) { Some((id, t)) } else { None })
            .collect()
    }

    pub fn all(&self) -> Vec<(u32, &Task)> {
        self.tasks.iter().map(|(&id, t)| (id, t)).collect()
    }

    pub fn running_tasks(&self) -> Vec<(u32, &Task)> {
        self.filter(|_, t| !t.is_completed())
    }

    pub fn completed_tasks(&self) -> Vec<(u32, &Task)> {
        self.filter(|_, t| t.is_completed())
    }
}

fn default_path() -> PathBuf {
    Path::new("").to_path_buf()
}
