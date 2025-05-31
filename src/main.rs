use std::{fs, io, process};
use std::fs::{File, };
use std::io::{stdout};
use std::path::Path;
use serde::{Serialize, Deserialize};
use crossterm::{execute,cursor, terminal::{Clear, ClearType} };
use press_btn_continue;
use crossterm::style::Stylize;

const FILENAME: &str = "todo.toml";

#[derive(Deserialize, Serialize, Clone)]
struct Notebook {
    #[serde(default)]
    notes: Vec<Note>,
}
#[derive(Deserialize, Serialize, Clone)]
struct Note {
    title: String,
    complete: bool,
}

fn main(){
    check();
    let todo_list = fs::read_to_string(FILENAME).expect("Unable to read file");
    let mut notebook: Notebook = toml::from_str(&todo_list).expect("Error parsing file");
    println!("{}", Stylize::cyan("MY NOTEBOOK BRUH").bold());
    loop {
        list_of_options();
        let choice = handle_choice();

        match choice {
            1 => {
                terminal_clear();
                println!("Title of note:");
                let title = handle_title();
                println!("Is note finished? (y/n)");
                let completed = handle_complete();
                new_note(&mut notebook, &*title, completed);
                press_btn_continue::wait("Press any key to continue...").unwrap();
            },
            2 => {
                terminal_clear();
                change_completion(&mut notebook);
            },
            3 => {
                terminal_clear();
                delete_note(&mut notebook);
                press_btn_continue::wait("Press any key to continue...").unwrap();
            },
            4 => {
                terminal_clear();
                list_notes(&notebook);
                press_btn_continue::wait("Press any key to continue...").unwrap();
            } 5  => {
                write_file(&notebook).expect("UHOH");
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

fn write_file(notebook: &Notebook) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(&notebook)?;
    fs::write(FILENAME, toml_string)?;
    Ok(())
}

fn new_note(notebook: &mut Notebook, title: &str, complete: bool) {
    let note = Note {
        title: title.to_string(),
        complete,
    };
    notebook.notes.push(note);
}
fn list_of_options() {
    let options: [&str; 6] = ["New Note", "Complete/Uncomplete Note", "Delete Note", "List Notes","Save notes","Exit"];
    for i in 0..options.len(){
        println!("({:?}) {}",i+1, options[i]);
    }
    println!("Choose an option:");
}
fn handle_choice() -> u32{
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading line");
    let choice=  choice.trim().parse::<u32>().unwrap_or(0);
    choice
}
fn handle_title() -> String {
    let mut title: String = String::new();
    io::stdin().read_line(&mut title).expect("Error reading line");
    let title = title.trim().to_string();
    title
}
fn handle_complete() -> bool {
    let mut complete: String = String::new();
    loop {
        io::stdin().read_line(&mut complete).expect("Error reading line");
        let complete = complete.trim().to_string().to_lowercase();
        if complete == "yes" || complete == "y" {break true;}
        else if complete == "no" || complete == "n" {break false;}
        else {println!("Not a valid option (y/n)");}
    }
}
fn terminal_clear() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();
}
fn delete_note(notebook: &mut Notebook) {
    list_notes(notebook);
    println!("Which do you want to delete?");
    loop {
        let choice = handle_choice();
        if choice > notebook.notes.len() as u32 {
            println!("No files to delete");
            press_btn_continue::wait("Press any key to continue...").unwrap();
            break;
        } else if choice > 0 {
            let choice = choice -1;
            notebook.notes.remove(choice as usize);
            println!("Note number {} successfully removed.",choice+1);
            break;
        } else {
            println!("Invalid input");
        }
    }
}
fn list_notes(notebook: &Notebook) {
    for (index, note) in notebook.notes.iter().enumerate() {
        println!("{}, {}, {}", index +1, note.title, note.complete);
    };
}

fn change_completion(notebook: &mut Notebook) {
    list_notes(notebook);
    loop {
        println!("Choose an option:");
        let choice = handle_choice();
        if choice > 0{
            let usize_choice = choice as usize - 1;
            let changed_completion = handle_complete();
            notebook.notes[usize_choice].complete = changed_completion;
            press_btn_continue::wait("Press any key to continue...").unwrap();
            break;
        } else {
            println!("Invalid input");
        }
    }
}