use std::io;

fn main_menu() -> String {
    println!("To-do Application");
    println!("Insert an option:");
    println!("1. Show pending tasks");
    println!("2. Add a new task");
    println!("3. Remove a task");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to option");

    guess
}

fn main() {
    let opt = main_menu();

    println!("{opt}")
}
