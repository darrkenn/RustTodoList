use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use crate::Taskbook;

pub fn list_of_options() {
    //Array for options
    let options: [&str; 6] = ["New Task", "Complete/Uncomplete Task", "Delete Task", "List Tasks","Save Tasks","Exit"];
    for i in 0..options.len(){
        println!("({:?}) {}",i+1, options[i]);
    }
    println!("Choose an option:");
}

pub fn list_tasks(taskbook: &Taskbook) {
    let mut table = Table::new();
    //Vector with attributes
    let header_attrib = vec![Attribute::Bold, Attribute::Fraktur];
    //Creates a table with UTF8_FULL preset and headers.
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Number").add_attributes(header_attrib.clone()).fg(Color::Cyan),
            Cell::new("Title").add_attributes(header_attrib.clone()).fg(Color::Cyan),
            Cell::new("Complete").add_attributes(header_attrib.clone()).fg(Color::Cyan),
        ]);

    /*For loop that iterates taskbook.tasks for each task. It then pushes each part into the vector new_row.
    This vector is then cloned and added as a new row. Then cleared to allow for more rows.*/
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
    //If there is no rows then it prints no notes found. If there is it prints the table
    if table.row_count() == 0 {
        println!("No notes found");
    } else {
        println!("{}", {table});
    }
}