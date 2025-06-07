use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use crate::keyhandling::{handle_guide_mode, handle_key, handle_new};
use crate::render::render;
use crate::Taskbook;

pub fn run(mut terminal: DefaultTerminal, taskbook: &mut Taskbook) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| render(f, taskbook))?;
        //Event reading
        if let Event::Key(key) = event::read()? {
            if taskbook.is_add_new_task {
                handle_new(key, taskbook);
            } else if  taskbook.is_guide {
                handle_guide_mode(key, taskbook);
            } else {
                if handle_key(key, taskbook) {
                    break;
                }
            }
        }
    }
    Ok(())
}