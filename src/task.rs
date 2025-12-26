use std::{fs::{File, OpenOptions}, io, path::{Path}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String, // task description
    pub completed: bool, // task status completed or not
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }
}

// Load Tasks from JSON File
pub fn load_tasks(path: &Path) -> Result<Vec<Task>, io::Error> {
    if path.exists() {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?)
    } else {
        Ok(Vec::new()) // Empty Vector
    }
}

// Save Task to JSON File
pub fn save_tasks(task: &[Task], path: &Path) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    serde_json::to_writer_pretty(file, task).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    Ok(())
}

// Add a New Task
pub fn add_task(description: String, tasks: &mut Vec<Task>, path: &Path) {
    let task = Task::new(description);
    tasks.push(task);
    let _ = save_tasks(&tasks, path);
    println!("Tasks Added Successfully");
}

// List All Tasks
pub fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No Tasks Found");
        return;
    }
    println!("List of Tasks:");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed {"\u{2713}"} else {" "};
        println!("{}: [{}] {}", index + 1, status, task.description);
    }
}

// Mark a Task Complete
pub fn complete_task (index: usize, tasks: &mut Vec<Task>, path: &Path) {
    if let Some(task) = tasks.get_mut(index - 1) {
        task.completed = true;
        let _ = save_tasks(&tasks, path);
        println!("Task Marked Complete");
    } else {
        println!("Invalid Index");
    }
}

// Delete a Task
pub fn delete_task (index: usize, tasks: &mut Vec<Task>, path: &Path) {
    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
        let _ = save_tasks(tasks, path);
        println!("Task Deleted Successfully");
    } else {
        println!("Invalid Index");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_task() {
        let mut tasks: Vec<Task> = Vec::new();
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        add_task("Buy Medicines".to_string(), &mut tasks, path);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Buy Medicines");
        assert!(!tasks[0].completed);
    }

    #[test]
    fn test_complete_task() {
        let mut tasks = vec![Task::new("Learn Rust".to_string())];
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        complete_task(1, &mut tasks, path);
        assert!(tasks[0].completed);
    }

    #[test]
    fn test_delete_task() {
        let mut tasks = vec![
            Task::new("Pack Things".to_string()),
            Task::new("Write Letters".to_string()),
        ];
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        delete_task(1, &mut tasks, path);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Write Letters");
    }

    #[test]
    fn test_save_load() {
        let mut tasks: Vec<Task> = Vec::new();
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        add_task("Persistent Task".to_string(), &mut tasks, path);

        let loaded = load_tasks(path).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(tasks[0].description, "Persistent Task");
    }
}