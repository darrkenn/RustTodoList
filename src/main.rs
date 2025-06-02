mod toml_check;
mod handle;
mod list;
mod todofuncs;

use std::{fs,process};
use std::io::{stdout};
use serde::{Serialize, Deserialize};
use press_btn_continue;
use crossterm::{execute,cursor};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use crate::handle::{handle_choice,handle_title};
use crate::list::{list_of_options, list_tasks};
use crate::todofuncs::{change_completion, delete_task, new_task};
use crate::toml_check::{check, write_file};

const FILENAME: &str = "todo.toml";
//Struct for Taskbook which has a vector containing the struct for a task.
#[derive(Deserialize, Serialize, Clone)]
struct Taskbook {
    #[serde(default)]
    tasks: Vec<Task>,
}
// Struct for a single task
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Task {
    title: String,
    complete: bool,
}
fn main(){
    //Checks if toml is present, if not it is automatically created.
    check();
    /*Reads the input from toml and saves it as string to todo_list.
    It then parses the TOML string into the struct Taskbook.*/
    let todo_list = fs::read_to_string(FILENAME).expect("Unable to read file");
    let mut taskbook: Taskbook = toml::from_str(&todo_list).expect("Error parsing file");
    
    loop {
        println!("{}", Stylize::cyan("To-Do List").bold());
        list_of_options();
        let choice = handle_choice();
        //Conditions for each choice.
        match choice {
            1 => {
                terminal_clear();
                println!("Name of task:");
                let title = handle_title();
                //Calls new task and passes a mutable reference to the taskbook and the users title.
                new_task(&mut taskbook, &*title);
                println!("Task '{}' successfully created.", title);
                press_btn_continue::wait("Press any key to continue").unwrap();
            },
            2 => {
                terminal_clear();
                change_completion(&mut taskbook);
            },
            3 => {
                terminal_clear();
                delete_task(&mut taskbook);
                press_btn_continue::wait("Press any key to continue...").unwrap();
            },
            4 => {
                terminal_clear();
                list_tasks(&taskbook);
                press_btn_continue::wait("Press any key to continue...").unwrap();
            } 5  => {
                write_file(&taskbook).expect("UHOH");
                println!("Successfully saved!");
                press_btn_continue::wait("Press any key to continue...").unwrap();
            } 6 => {
                process::exit(0);
            }
            _ => {}
        }
        terminal_clear();
    }
}
//Terminal clear function as im too lazy to type it out each time.
fn terminal_clear() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();
}





