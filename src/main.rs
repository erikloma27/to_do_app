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

fn print_task() {
    println!("Print task!")

}

fn add_task() {
    println!("Add task!")

}

fn remove_task() {
    println!("Remove task!")

}

fn main() {
    let opt = main_menu();
    let opt: i8 = opt.trim().parse().expect("Type a valid number!");

    match opt {
        1 => print_task(),
        2 => add_task(),
        3 => remove_task(),
        _ => println!("Invalid option"), 
    };
}
