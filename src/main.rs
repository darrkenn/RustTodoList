use ratatui::prelude::Stylize;

mod toml_check;

use std::{fs};
use serde::{Serialize, Deserialize};
use press_btn_continue;
use crossterm::{event};
use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget};
use crate::toml_check::{check, write_file};
use ratatui::text::ToSpan;

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
    let mut taskbook: Taskbook = toml::from_str(&todo_list).expect("Error parsing file");

    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut taskbook);

    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    result
}
fn run(mut terminal: DefaultTerminal, taskbook: &mut Taskbook) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, taskbook))?;
            if let Event::Key(key) = event::read()? {
                if taskbook.is_add_new_task {
                    match handle_new_task(key, taskbook) {
                        TaskAction::Submit => {
                            taskbook.is_add_new_task = false;
                            if taskbook.input_title.is_empty() {

                            } else {
                                taskbook.tasks.push(Task {
                                    title: taskbook.input_title.clone(),
                                    complete: false,
                                });
                                taskbook.input_title.clear();
                            }
                        }
                        TaskAction::Exit => {
                            taskbook.is_add_new_task = false;
                            taskbook.input_title.clear();
                        }
                        TaskAction::Nothing => {}
                    }
                } else {
                    if handle_key(key, taskbook) {
                        break;
                    }
                }
            }

    }
    Ok(())
}

fn handle_new_task(key: KeyEvent, taskbook: &mut Taskbook) -> TaskAction {
    if key.kind != KeyEventKind::Press {
        return TaskAction::Nothing;
    }
    match key.code {
        event::KeyCode::Tab => {return TaskAction::Exit;}
        event::KeyCode::Enter => {return TaskAction::Submit;}

        event::KeyCode::Char(c) => {
            taskbook.input_title.push(c);
        }
        event::KeyCode::Backspace => {
            taskbook.input_title.pop();
        }

        _ => {return TaskAction::Nothing}
    }
    TaskAction::Nothing
}
fn handle_key(key:KeyEvent, taskbook: &mut Taskbook) -> bool{
    if key.kind != KeyEventKind::Press {
        return false;
    }
        match key.code {
            event::KeyCode::Esc => {
                return true;
            }
            event::KeyCode::Enter => {
                if let Some(index) = taskbook.task_state.selected() {
                    taskbook.tasks[index].complete = true;
                }
            }
            event::KeyCode::Up => {taskbook.task_state.select_previous()},
            event::KeyCode::Down => {taskbook.task_state.select_next()},
            event::KeyCode::Char(char) => match char {
                'N' => {
                    taskbook.is_add_new_task = true;
                }
                'D' => {
                    if let Some(index) = taskbook.task_state.selected() {
                        taskbook.tasks.remove(index);
                    }
                }
                'C' => {
                    write_file(&taskbook).expect("Err")
                }
                'w' => {
                    taskbook.task_state.select_previous();
                }
                's' => {
                    taskbook.task_state.select_next();
                }
                _ => {}
            }
            _ => {}
        }
    false
}

fn render(frame: &mut Frame, taskbook: &mut Taskbook) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    if taskbook.is_add_new_task {
        Paragraph::new(taskbook.input_title.as_str())
            .block(
                Block::bordered()
                    .title("Enter title".to_span().into_centered_line().bold().cyan())
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded))
            .render(frame.area(), frame.buffer_mut());
    } else {


        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(Color::LightBlue)
            .render(border_area, frame.buffer_mut());

        let tasks: Vec<ListItem> = taskbook
            .tasks
            .iter()
            .map(|task| {
                let style = if task.complete {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD)
                };
                ListItem::from(task.title.clone()).style(style)
            })
            .collect();

        let tasklist = List::new(tasks)
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Yellow));

        frame.render_stateful_widget(tasklist, inner_area, &mut taskbook.task_state);
    }
}







