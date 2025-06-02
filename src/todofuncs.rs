use crate::{Task, Taskbook};
use crate::handle::{handle_choice, handle_complete};
use crate::list::list_tasks;

pub fn new_task(taskbook: &mut Taskbook, title: &str) {
    //Creates new task with title as users selected and automatically sets complete to false.
    let task = Task {
        title: title.to_string(),
        complete: false,
    };
    //Pushes to taskbook.tasks vector.
    taskbook.tasks.push(task);
}

pub fn delete_task(taskbook: &mut Taskbook) {
    list_tasks(taskbook);
    //Firstly checks if length is == 0, if so it automatically shuts down.
    if taskbook.tasks.len() == 0 {
        press_btn_continue::wait("Press any key to continue...").unwrap();
    } else {
        println!("Which do you want to delete?");
        loop {
            let choice = handle_choice();
            if choice > taskbook.tasks.len() as u32 {
                println!("There is no task number {}",choice);
                break;
            } else if choice > 0 {
                let choice = choice - 1;
                taskbook.tasks.remove(choice as usize);
                println!("Note number {} successfully removed.", choice + 1);
                break;
            } else {
                println!("Invalid input");
            }
        }
    }
}

pub fn change_completion(taskbook: &mut Taskbook) {
    list_tasks(taskbook);
    //First checks if length is == to 0.
    if taskbook.tasks.len() == 0 {
        press_btn_continue::wait("Press any key to continue...").unwrap();
    } else {
        loop {
            println!("Which note would you like to change? (0) to exit");
            let choice = handle_choice();
            //Checks if choice is greater than 0 and less than length of taskbook.tasks.
            if choice > 0 && choice <  taskbook.tasks.len() as u32 + 1 {
                println!("Now changing task {}", choice);
                let usize_choice = choice as usize - 1;
                //Changledcompletition is then pushed to the vector taskbook.tasks based on the users choice.
                let changed_completion = handle_complete();
                taskbook.tasks[usize_choice].complete = changed_completion;
                press_btn_continue::wait("Press any key to continue...").unwrap();
                break;
            } else if choice == 0 {
                break;
            } else {
                println!("Invalid input");
            }
        }
    }
}