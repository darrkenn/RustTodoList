use ratatui::prelude::Stylize;
mod handle;
mod list;
mod todofuncs;
mod toml_check;

use std::{fs, io, process};
use std::io::{stdout};
use serde::{Serialize, Deserialize};
use press_btn_continue;
use crossterm::{execute, cursor, event};
use crossterm::terminal::{Clear, ClearType};
use color_eyre::eyre::Result;
use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame, Terminal};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color};
use ratatui::widgets::{Block, BorderType, List, ListItem, Paragraph, Widget};
use crate::handle::{handle_choice, handle_title};
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


fn main() -> Result<()>{
    //Checks if toml is present, if not it is automatically created.
    check();


    /*Reads the input from toml and saves it as string to todo_list.
    It then parses the TOML string into the struct Taskbook.*/
    let todo_list = fs::read_to_string(FILENAME).expect("Unable to read file");
    let mut taskbook: Taskbook = toml::from_str(&todo_list).expect("Error parsing file");

    color_eyre::install()?;
    
    let terminal = ratatui::init();
    let result = run(terminal, &mut taskbook);
    ratatui::restore();
    
    // loop {
    //     println!("{}", Stylize::cyan("To-Do List").bold());
    //     list_of_options();
    //     let choice = handle_choice();
    //     //Conditions for each choice.
    //     match choice {
    //         1 => {
    //             terminal_clear();
    //             println!("Name of task:");
    //             let title = handle_title();
    //             //Calls new task and passes a mutable reference to the taskbook and the users title.
    //             new_task(&mut taskbook, &*title);
    //             println!("Task '{}' successfully created.", title);
    //             press_btn_continue::wait("Press any key to continue").unwrap();
    //         },
    //         2 => {
    //             terminal_clear();
    //             change_completion(&mut taskbook);
    //         },
    //         3 => {
    //             terminal_clear();
    //             delete_task(&mut taskbook);
    //             press_btn_continue::wait("Press any key to continue...").unwrap();
    //         },
    //         4 => {
    //             terminal_clear();
    //             list_tasks(&taskbook);
    //             press_btn_continue::wait("Press any key to continue...").unwrap();
    //         } 5  => {
    //             write_file(&taskbook).expect("UHOH");
    //             println!("Successfully saved!");
    //             press_btn_continue::wait("Press any key to continue...").unwrap();
    //         } 6 => {
    //             process::exit(0);
    //         }
    //         _ => {}
    //     }
    //     terminal_clear();
    // }
    result
}

fn run(mut terminal: DefaultTerminal, taskbook: &mut Taskbook) -> Result<()> {
    loop {
        terminal.draw(|f| render( f, taskbook) )?;
        //Exit handle
        if let Event::Key(key) = event::read()?{
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }

    }
    Ok(())
}

fn render(frame:&mut Frame, taskbook:&Taskbook) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);


    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::LightBlue)
        .render(border_area, frame.buffer_mut());

    List::new(
        taskbook
            .tasks
            .iter()
            .map(|x| ListItem::from(x.title.clone())))
        .render(inner_area, frame.buffer_mut());

    //Paragraph::new("Yoooo im rust").render(frame.area(), frame.buffer_mut());
}



//Terminal clear function as im too lazy to type it out each time.
fn terminal_clear() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();
}



