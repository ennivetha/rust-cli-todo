mod task;
use std::path::{Path};

use task::{add_task, list_tasks, complete_task, delete_task, load_tasks};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", version = "1.0", about = "Simple CLI Todo App")]
struct TodoCli {
    #[command(subcommand)]
    todo_command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    List,
    Complete { index: usize },
    Delete { index: usize },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = TodoCli::parse();
    let file_path = Path::new("tasks.json");
    let mut tasks = load_tasks(file_path)?;
    
    match cli.todo_command {
        Command::Add { description } => add_task(description, &mut tasks, file_path),
        Command::List => list_tasks(&tasks),
        Command::Complete { index } => complete_task(index, &mut tasks, file_path),
        Command::Delete { index } => delete_task(index, &mut tasks, file_path),
    }
    Ok(())
}
