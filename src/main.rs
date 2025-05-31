use std::{fs, io, process};
use std::fs::{File, };
use std::io::{stdout};
use std::path::Path;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use serde::{Serialize, Deserialize};
use crossterm::{execute,cursor, terminal::{Clear, ClearType} };
use press_btn_continue;
use crossterm::style::Stylize;
use comfy_table::{Attribute, ContentArrangement, Table};
use comfy_table::*;

const FILENAME: &str = "todo.toml";
#[derive(Deserialize, Serialize, Clone)]
struct Taskbook {
    #[serde(default)]
    tasks: Vec<Task>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Task {
    title: String,
    complete: bool,
}


fn main(){
    check();
    let todo_list = fs::read_to_string(FILENAME).expect("Unable to read file");
    let mut taskbook: Taskbook = toml::from_str(&todo_list).expect("Error parsing file");

    loop {
        println!("{}", Stylize::cyan("To-Do List").bold());
        list_of_options();
        let choice = handle_choice();

        match choice {
            1 => {
                terminal_clear();
                println!("Name of task:");
                let title = handle_title();
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
fn check(){
    if !Path::new(FILENAME). exists() {
        File::create("todo.toml").unwrap();
    }
}

fn write_file(taskbook: &Taskbook) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(&taskbook)?;
    fs::write(FILENAME, toml_string)?;
    Ok(())
}

fn new_task(taskbook: &mut Taskbook, title: &str) {
    let note = Task {
        title: title.to_string(),
        complete: false,
    };
    taskbook.tasks.push(note);
}
fn list_of_options() {
    let options: [&str; 6] = ["New Task", "Complete/Uncomplete Task", "Delete Task", "List Tasks","Save Tasks","Exit"];
    for i in 0..options.len(){
        println!("({:?}) {}",i+1, options[i]);
    }
    println!("Choose an option:");
}
fn handle_choice() -> u32 {
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading line");
    let choice=  choice.trim().parse::<u32>().unwrap_or(0);
    choice
}
fn handle_title() -> String {
    let mut title: String = String::new();
    loop {
        io::stdin().read_line(&mut title).expect("Error reading line");
        let title = title.trim().to_string();
        if title.is_empty() {
                println!("Title cant be empty");
        } else {
            break title;
        }
    }
}
fn handle_complete() -> bool {
    let mut complete: String = String::new();
    loop {
        complete.clear();
        io::stdin().read_line(&mut complete).expect("Error reading line");
        let complete = complete.trim().to_string().to_lowercase();
        if complete == "yes" || complete == "y" {
            break true;
        } else if complete == "no" || complete == "n" {
            break false;
        } else {
            println!("{} is Not a valid option (y/n)", complete);
        }
    }
}
fn terminal_clear() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();
}
fn delete_task(taskbook: &mut Taskbook) {
    list_tasks(taskbook);
    if taskbook.tasks.len() == 0 {
        press_btn_continue::wait("Press any key to continue...").unwrap();
    } else {
        println!("Which do you want to delete?");
        loop {
            let choice = handle_choice();
            if choice > taskbook.tasks.len() as u32 {
                println!("No files to delete");
                press_btn_continue::wait("Press any key to continue...").unwrap();
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
fn list_tasks(taskbook: &Taskbook) {
    let mut table = Table::new();
    let header_attrib = vec![Attribute::Bold, Attribute::Fraktur];
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Number").add_attributes(header_attrib.clone()).fg(Color::Cyan),
            Cell::new("Title").add_attributes(header_attrib.clone()).fg(Color::Cyan),
            Cell::new("Complete").add_attributes(header_attrib.clone()).fg(Color::Cyan),
        ]);


    let mut new_row: Vec<String> = Vec::new();
    for (index, note) in taskbook.tasks.iter().enumerate() {
        let num = index +1;
        let num: String = num.to_string();
        new_row.push(num);
        new_row.push(note.title.clone());
        new_row.push(note.complete.to_string());
        table.add_row(new_row.clone());
        new_row.clear();
    };
    if table.row_count() == 0 {
        println!("No notes found");
    } else {
        println!("{}", {table});
    }
}

fn change_completion(taskbook: &mut Taskbook) {
    list_tasks(taskbook);
    if taskbook.tasks.len() == 0 {
        press_btn_continue::wait("Press any key to continue...").unwrap();
    } else {
        loop {
            println!("Which note would you like to change? (0) to exit");
            let choice = handle_choice();

            if choice > 0 {
                println!("Now changing note {}", choice);
                let usize_choice = choice as usize - 1;
                let changed_completion = handle_complete();
                taskbook.tasks[usize_choice].complete = changed_completion;
                press_btn_continue::wait("Press any key to continue...").unwrap();
                break;
            } else if choice == 0 {
                println!("Exiting");
                break;
            } else {
                println!("Invalid input");
            }
        }
    }
}