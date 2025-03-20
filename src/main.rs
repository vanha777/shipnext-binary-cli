use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Example: Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./shipnext <command>");
        println!("Examples:");
        println!("  ./shipnext status  - Check git status");
        println!("  ./shipnext modify  - Modify a file");
        return;
    }

    match args[1].as_str() {
        "status" => run_command("git", &["status"]),
        "modify" => modify_file(),
        cmd => println!("Unknown command: {}", cmd),
    }
}

// Function to run a terminal command
fn run_command(cmd: &str, args: &[&str]) {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

// Function to modify a file
fn modify_file() {
    let file_path = "example.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");

    writeln!(file, "Modified by shipnext at {}", chrono::Utc::now())
        .expect("Failed to write to file");
    println!("File {} modified successfully!", file_path);

    // Run a command to show the change
    run_command("cat", &[file_path]);
}