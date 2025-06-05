mod toml_check;

use std::{fs};
use serde::{Serialize, Deserialize};
use crossterm::{event};
use color_eyre::eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Widget, Clear, Wrap};
use crate::toml_check::{check, write_file};
use ratatui::text::{Line};


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

    //Initialise terminal and stuff
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
        //Event reading
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
                } else if  taskbook.is_guide {
                    match handle_guide(key) {
                        false => {
                            taskbook.is_guide = false;
                        }
                        _ => {}
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
    //Checks if keyevenykind isnt a press
    if key.kind != KeyEventKind::Press {
        return TaskAction::Nothing;
    }
    //Key checking again
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

fn handle_guide(key: KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return true;
    }
    return match key.code {
        event::KeyCode::Tab => { false }

        _ => { true }
    }
}
fn handle_information(key: KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return true;
    }
    return match key.code {
        event::KeyCode::Tab => { false }

        _ => { true }
    }
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
                'n' => {
                    taskbook.is_add_new_task = true;
                }
                'g' => {
                    taskbook.is_guide = true;
                }
                'd' => {
                    //Removes value based on task state
                    if let Some(index) = taskbook.task_state.selected() {
                        taskbook.tasks.remove(index);
                    }
                }
                'c' => {
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
    //Creates a split in the screen
    let [border_area] = Layout::horizontal([Constraint::Ratio(50,100)  ])
        .margin(1)
        .areas(frame.area());

    let area = frame.area();
    let chunks = Layout::new (
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)]
    ).split(area);

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        //Block that contains tasks
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White))
            .title_style(Color::Cyan)
            .title(Line::from("RustTodoList").centered())
            .title_bottom(Line::from("Guide(g)").centered())
            .title_bottom(Line::from("Exit(Esc)").centered())
            .render(border_area, frame.buffer_mut());

        //Vector of tasks in the tui
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
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::LightMagenta))
            .highlight_spacing(HighlightSpacing::Always);

    if taskbook.is_add_new_task {
        //Input title
        let input_title = Paragraph::new(taskbook.input_title.as_str()).centered().wrap(Wrap { trim: true });
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::White))
            .title_style(Color::Cyan)
            .style(Style::default().fg(Color::Green))
            .title_bottom(Line::from("Quit(Tab) Save(Enter)").centered())
            .title(Line::from("Enter title").centered());
        //Epands area based on length of text
        let mut input_y = 10;
        if taskbook.input_title.len() > 58 {
            input_y += 5;
        } else {

        }
        let area = popup_area(chunks[1], 90, input_y as u16);
        Paragraph::new(taskbook.input_title.as_str());
        frame.render_widget(Clear, area);
        frame.render_widget(input_title.block(block), area);

    } else if taskbook.is_guide {
        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .bg(Color::Rgb(18, 18, 18))
            .border_style(Color::White)
            .title_style(Color::Cyan)
            .title(Line::from("Guide").centered()).cyan()
            .title_bottom(Line::from("Back(tab)").centered());
        let options = [
            "New task (n)", "Delete Task (d)",
            "Up (w)", "Down (s)",
            "Save (c)",
            "Complete task (Enter)",
            "Back (Tab)", "Exit (Esc)"
        ];
        let list = List::new(options)
            .block(block).bold().green();

        let area = popup_area(chunks[1], 40, 36);
        frame.render_widget(Clear, area);
        frame.render_widget(list, area);
    }
    
    frame.render_stateful_widget(tasklist, inner_area, &mut taskbook.task_state);
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    //Calculation for popup to be centered
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}







