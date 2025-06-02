use std::io;

pub fn handle_choice() -> u32 {
    //Handles input as string and then converts it to u32 and returns it
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading line");
    let choice=  choice.trim().parse::<u32>().unwrap_or(0);
    choice
}
pub fn handle_title() -> String {
    let mut title: String = String::new();
    loop {
        io::stdin().read_line(&mut title).expect("Error reading line");
        let title = title.trim().to_string();
        //Checks if string is empty, if so loop isnt broken.If not loop is broken and title is returned.
        if title.is_empty() {
            println!("Title cant be empty");
        } else {
            break title;
        }
    }
}

pub fn handle_complete() -> bool {
    let mut complete: String = String::new();
    loop {
        //Clears complete idk why it just didnt clear anywhere else.
        complete.clear();
        io::stdin().read_line(&mut complete).expect("Error reading line");
        let complete = complete.trim().to_string().to_lowercase();
        //Checks if complete is either yes or no. Returns with respective outcome.
        if complete == "yes" || complete == "y" || complete == "true" {
            break true;
        } else if complete == "no" || complete == "n" || complete == "false" {
            break false;
        } else {
            println!("{} is Not a valid option (y/n)", complete);
        }
    }
}