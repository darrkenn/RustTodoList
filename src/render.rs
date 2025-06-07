use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, ListItem, Padding, Paragraph, Widget, Clear, Wrap};
use ratatui::text::{Line};
use crate::Taskbook;

const OPTIONS: [&str; 8] = [
"New task (n)", "Delete Task (d)",
"Up (w)", "Down (s)",
"Save (c)",
"Complete task (Enter)",
"Back (Tab)", "Exit (Esc)"
];

pub fn render(frame: &mut Frame, taskbook: &mut Taskbook) {
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
        .title(Line::from("RusTuiList").centered())
        .title_bottom(Line::from("New(n)").centered())
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
            let is_complete = if task.complete {
                "[*]"
            } else {
                "[ ]"
            };
            let complete = format!("{} {}",is_complete,task.title);
            ListItem::from(complete).style(style)
        })
        .collect();

    let tasklist = List::new(tasks)
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::default());

    if taskbook.is_add_new_task {
        new_task_render(taskbook,frame,chunks[1]);
    } else if taskbook.is_guide {
        guide_render(frame, chunks[1]);
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

fn new_task_render(taskbook: &mut Taskbook, frame: &mut Frame, chunk_area: Rect) {
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
    let area = popup_area(chunk_area, 90, input_y as u16);
    Paragraph::new(taskbook.input_title.as_str());
    frame.render_widget(Clear, area);
    frame.render_widget(input_title.block(block), area);
}

fn guide_render(frame: &mut Frame, chunk_area: Rect) {
    let block = Block::bordered()
        .padding(Padding::uniform(1))
        .border_style(Color::White)
        .title_style(Color::Cyan)
        .title(Line::from("Guide").centered()).cyan()
        .title_bottom(Line::from("Back(tab)").centered());

    let list = List::new(OPTIONS)
        .block(block).bold().green();

    let area = popup_area(chunk_area, 40, 27);
    frame.render_widget(Clear, area);
    frame.render_widget(list, area);
}


