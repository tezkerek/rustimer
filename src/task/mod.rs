use anyhow::Error;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap, fmt::Debug, fs::OpenOptions, io, io::BufReader,
    path::Path, path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
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
        self.end_time != None
    }

    /// Get elapsed time until end if completed, or until now if ongoing
    pub fn elapsed(&self) -> Duration {
        match self.end_time {
            Some(end) => end - self.start_time,
            None => Utc::now() - self.start_time,
        }
    }

    pub fn complete_now(&mut self) {
        self.end_time = Some(Utc::now())
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskStore {
    tasks: BTreeMap<u32, Task>,

    #[serde(skip_serializing, default = "default_path")]
    save_path: PathBuf,
}

impl TaskStore {
    pub fn from_file(path: &Path) -> Result<Self, Error> {
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

    pub fn save(&self) -> Result<(), Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.save_path)?;

        serde_json::to_writer(file, self)?;

        Ok(())
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

    pub fn update(&mut self, id: u32, task: Task) -> &Task {
        self.tasks.insert(id, task);
        return self.tasks.get(&id).unwrap();
    }

    pub fn get_by_id(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn active_tasks(&self) -> Vec<(u32, &Task)> {
        self.tasks
            .iter()
            .filter_map(|(&id, t)| {
                if t.end_time == None {
                    Some((id, t))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn default_path() -> PathBuf {
    Path::new("").to_path_buf()
}
