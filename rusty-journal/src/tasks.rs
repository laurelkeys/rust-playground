use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Self {
        Task {
            text,
            created_at: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Seek, SeekFrom};
use std::path::PathBuf;

/// Parse the given `file` into a vector of tasks.
fn collect_tasks(mut file: &File) -> io::Result<Vec<Task>> {
    let initial_offset = file.seek(SeekFrom::Current(0))?;

    file.seek(SeekFrom::Start(0))?; // rewind the file before reading from it
    let tasks = serde_json::from_reader(file);
    file.seek(SeekFrom::Start(initial_offset))?; // restore the cursor offset

    tasks.or_else(|e| {
        if e.is_eof() {
            Ok(Vec::new())
        } else {
            Err(e.into())
        }
    })
}

/// https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/5-add-task-function
pub fn add_task(journal_path: PathBuf, task: Task) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

/// https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/6-complete-task-function
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;
    let tasks_len = tasks.len();

    // Note: the task position follows 1-based indexing.
    if (1..=tasks_len).contains(&task_position) {
        tasks.remove(task_position - 1);

        file.set_len(0)?;
        serde_json::to_writer(file, &tasks)?;

        Ok(())
    } else {
        Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Invalid task position: {} (the journal has {} {})",
                task_position,
                tasks_len,
                if tasks_len == 1 { "task" } else { "tasks" }
            ),
        ))
    }
}

/// https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/7-list-tasks-function
pub fn list_tasks(journal_path: PathBuf) -> io::Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;

    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("The task list is empty");
    } else {
        for (position, task) in tasks.iter().enumerate() {
            // Note: the task position follows 1-based indexing.
            println!("{}: {}", position + 1, task);
        }
    }

    Ok(())
}
