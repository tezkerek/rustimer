use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{
    error::Error, fmt::Debug, fs::OpenOptions, io, io::BufReader, path::Path,
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
pub struct TaskStore {
    tasks: Vec<Task>,

    #[serde(skip_serializing, default = "default_path")]
    save_path: PathBuf,
}

impl TaskStore {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
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
                    tasks: Vec::new(),
                    save_path: path.to_path_buf(),
                },
                _ => return Err(Box::new(e)),
            },
        };

        Ok(store)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.save_path)?;

        serde_json::to_writer(file, self)?;

        Ok(())
    }

    pub fn add(&mut self, task: Task) -> &Task {
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    pub fn active_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.end_time == None).collect()
    }
}

fn default_path() -> PathBuf {
    Path::new("").to_path_buf()
}
