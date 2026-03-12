use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),           // Directory path
    Display(String),        // File path
    Create(String, String), // File path and content
    Remove(String),         // File path
    Pwd,                    // Print working directory
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory_path) => {
            let result = Command::new("ls").arg(&directory_path).status();

            match result {
                Ok(status) if status.success() => {}
                Ok(_) => eprintln!("Failed to list files in '{}'.", directory_path),
                Err(_) => eprintln!("Failed to execute ls command."),
            }
        }

        FileOperation::Display(file_path) => {
            let result = Command::new("cat").arg(&file_path).status();

            match result {
                Ok(status) if status.success() => {}
                Ok(_) => eprintln!("Failed to display file '{}'.", file_path),
                Err(_) => eprintln!("Failed to execute cat command."),
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            let result = Command::new("sh").arg("-c").arg(&command).status();

            match result {
                Ok(status) if status.success() => {
                    println!("File '{}' created successfully.", file_path);
                }
                Ok(_) => eprintln!("Failed to create file '{}'.", file_path),
                Err(_) => eprintln!("Failed to execute file creation command."),
            }
        }

        FileOperation::Remove(file_path) => {
            let result = Command::new("rm").arg(&file_path).status();

            match result {
                Ok(status) if status.success() => {
                    println!("File '{}' removed successfully.", file_path);
                }
                Ok(_) => eprintln!("Failed to remove file '{}'.", file_path),
                Err(_) => eprintln!("Failed to execute rm command."),
            }
        }

        FileOperation::Pwd => {
            let result = Command::new("pwd").status();

            match result {
                Ok(status) if status.success() => {}
                Ok(_) => eprintln!("Failed to print working directory."),
                Err(_) => eprintln!("Failed to execute pwd command."),
            }
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = read_input("Enter your choice (0-5): ");

        let operation = match choice.as_str() {
            "1" => {
                let directory = read_input("Enter directory path: ");
                Some(FileOperation::List(directory))
            }
            "2" => {
                let file_path = read_input("Enter file path: ");
                Some(FileOperation::Display(file_path))
            }
            "3" => {
                let file_path = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                Some(FileOperation::Create(file_path, content))
            }
            "4" => {
                let file_path = read_input("Enter file path: ");
                Some(FileOperation::Remove(file_path))
            }
            "5" => Some(FileOperation::Pwd),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number from 0 to 5.");
                None
            }
        };

        if let Some(op) = operation {
            perform_operation(op);
        }
    }
}