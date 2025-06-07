use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use crate::{Task, TaskAction, Taskbook};
use crate::toml_check::write_file;

pub fn handle_key(key:KeyEvent, taskbook: &mut Taskbook) -> bool{
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
            'n' | 'N' => {
                taskbook.is_add_new_task = true;
            }
            'g' | 'G' => {
                taskbook.is_guide = true;
            }
            'd' | 'D' => {
                //Removes value based on task state
                if let Some(index) = taskbook.task_state.selected() {
                    taskbook.tasks.remove(index);
                }
            }
            'c' | 'C' => {
                write_file(&taskbook).expect("Err")
            }
            'w' | 'W' => {
                taskbook.task_state.select_previous();
            }
            's' | 'S' => {
                taskbook.task_state.select_next();
            }
            _ => {}
        }
        _ => {}
    }
    false
}




pub fn handle_new(key: KeyEvent, taskbook: &mut Taskbook) {
    if key.kind != KeyEventKind::Press {
        taskbook.is_add_new_task = false;
    }
    match handle_new_task(key, taskbook) {
        TaskAction::Submit => {
            if taskbook.input_title.is_empty() {

            } else {
                handle_save_task(taskbook);
                taskbook.is_add_new_task = false;
            }
        }
        TaskAction::Exit => {
            exit_save_task(taskbook);
        }
        TaskAction::Nothing => {}
    }
}

pub fn handle_save_task(taskbook: &mut Taskbook) {
    taskbook.tasks.push(Task {
        title: taskbook.input_title.clone(),
        complete: false,
    });
    taskbook.input_title.clear();
}
pub fn exit_save_task(taskbook: &mut Taskbook) {
    taskbook.is_add_new_task = false;
    taskbook.input_title.clear();
}
pub fn handle_guide_mode(key: KeyEvent, taskbook: &mut Taskbook) {
    if !handle_guide(key) {
        taskbook.is_guide = false;
    }
}

pub fn handle_new_task(key: KeyEvent, taskbook: &mut Taskbook) -> TaskAction {
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

pub fn handle_guide(key: KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return true;
    }
    match key.code {
        event::KeyCode::Tab => { false }

        _ => { true }
    }
}