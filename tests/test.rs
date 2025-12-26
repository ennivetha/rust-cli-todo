use std::{fs::remove_file, process::{Command, Output}};

fn run_app(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rust-cli-todo"))
        .args(args)
        .output()
        .expect("Failed to Run the App")
}

#[test]
fn test_add_list() {
    let _ = remove_file("tasks.json");

    // Add a Task
    let output = run_app(&["add", "Learn Rust"]);
    assert!(output.status.success());

    // List the tasks
    let output = run_app(&["list"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("List Output:\n{}", stdout);

    assert!(stdout.contains("Learn Rust"));
    assert!(stdout.contains("[ ]"));
}

#[test]
fn test_complete_delete() {
    let _ = remove_file("tasks.json");

    // Add tasks
    let _ = run_app(&["add", "Buy Milk"]).status.success();
    let _ = run_app(&["add", "Buy Medicine"]).status.success();

    // Mark a Task Complete
    let _ = run_app(&["complete", "2"]).status.success();

    // Delete a Task
    let _ = run_app(&["delete", "1"]).status.success();

    // List the tasks
    let output = run_app(&["list"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("List Output:\n{}", stdout);

    assert!(stdout.contains("\u{2713}"));
    assert!(stdout.contains("Buy Medicine"));
}