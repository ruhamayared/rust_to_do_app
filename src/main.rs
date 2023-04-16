// Import necessary modules from the standard library
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
// stdin provides a way to read user input from the standard input stream (usually the keyboard)
// stdout provides access to the standard output stream (usually the console). We use stdout().flush() to ensure that any pending output is written to the console immediately.
// Write provides the flush() method for output streams, including stdout
// HashMap provides a HashMap data structure, which is a key-value store. In our To-Do app, we use a HashMap to store tasks with a unique identifier (u32) as the key and the task description (String) as the value.

fn main() {
    // Create a HashMap to store tasks with an ID as the key and the task description as the value
    let mut tasks: HashMap<u32, String> = HashMap::new();
    // Initialize a counter for task IDs
    let mut task_id_counter: u32 = 1;

    // Main loop to display the menu and handle user actions
    loop {
        println!("What would you like to do?");
        println!("1) Add task");
        println!("2) List tasks");
        println!("3) Complete task");
        println!("4) Quit");

        // Read the user's choice
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();

        // Match the user's choice with the corresponding action
        match choice.trim().parse::<u8>() {
            // Add a task
            Ok(1) => {
                let mut task = String::new();
                print!("Enter the task: ");
                stdout().flush().unwrap(); // Flush stdout to ensure the prompt is displayed before reading input
                stdin().read_line(&mut task).unwrap();

                // Insert the task into the HashMap with the current task_id_counter value as the key
                tasks.insert(task_id_counter, task.trim().to_string());
                println!("Task added with ID: {}", task_id_counter);
                task_id_counter += 1; // Increment the task_id_counter for the next task
            }
            // List tasks
            Ok(2) => {
                if tasks.is_empty() {
                    println!("No tasks.");
                } else {
                    println!("Tasks:");
                    // Iterate through the HashMap and print the task ID and description
                    // for (id, task) in &tasks {
                    //     println!("{} - {}", id, task);
                    // }

                    // Collect tasks into a Vec and sort by task ID
                    let mut sorted_tasks: Vec<(&u32, &String)> = tasks.iter().collect();
                    sorted_tasks.sort_by_key(|&(id, _)| id);

                    // Iterate through the sorted Vec and print the task ID and description
                    for (id, task) in sorted_tasks {
                        println!("{} - {}", id, task);
                    }
                }
            }
            // Complete (remove) a task
            Ok(3) => {
                let mut task_id = String::new();
                print!("Enter the task ID: ");
                stdout().flush().unwrap(); // Flush stdout to ensure the prompt is displayed before reading input
                stdin().read_line(&mut task_id).unwrap();

                // Try to parse the input as a u32 and remove the corresponding task from the HashMap
                match task_id.trim().parse::<u32>() {
                    Ok(id) => {
                        if tasks.remove(&id).is_some() {
                            println!("Task {} completed.", id);
                        } else {
                            println!("Task not found.");
                        }
                    }
                    Err(_) => println!("Invalid task ID."),
                }
            }
            // Quit the app
            Ok(4) => {
                println!("Goodbye!");
                break;
            }
            // Invalid choice
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}
