mod toml_check;
mod keyhandling;
mod render;
mod run;

use std::{fs};
use serde::{Serialize, Deserialize};
use color_eyre::eyre::Result;
use ratatui::widgets::ListState;
use crate::toml_check::check;
use crate::run::run;

const FILENAME: &str = "todo.toml";
//Struct for Taskbook which has a vector containing the struct for a task.
#[derive(Deserialize, Serialize, Clone)]
struct Taskbook {
    #[serde(default)]
    tasks: Vec<Task>,
    #[serde(skip)]
    task_state: ListState,
    #[serde(skip)]
    is_add_new_task: bool,
    #[serde(skip)]
    input_title: String,
    #[serde(skip)]
    is_guide: bool,
    #[serde(skip)]
    is_information: bool,
}
// Struct for a single task
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Task {
    title: String,
    complete: bool,
}
enum TaskAction {
    Nothing,
    Submit,
    Exit
}

fn main() -> Result<()>{
    //Checks if toml is present, if not it is automatically created.
    check();
    /*Reads the input from toml and saves it as string to todo_list.
    It then parses the TOML string into the struct Taskbook.*/
    let todo_list = fs::read_to_string(FILENAME).expect("Unable to read file");
    let mut taskbook: Taskbook = toml::from_str((&todo_list).as_ref()).expect("Error parsing file");

    //Initialise terminal and stuff
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut taskbook);

    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    result
}










