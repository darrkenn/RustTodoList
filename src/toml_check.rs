use std::fs;
use std::fs::File;
use std::path::Path;
use crate::{Taskbook, FILENAME};

pub fn check(){
    //Checks if toml doesnt exist, if so it is created.
    if !Path::new(FILENAME).exists() {File::create("todo.toml").unwrap();}
}
pub fn write_file(taskbook: &Taskbook) -> Result<(), Box<dyn std::error::Error>> {
    //Writes the struct to the toml.
    let toml_string = toml::to_string(&taskbook)?;
    fs::write(FILENAME, toml_string)?;
    Ok(())
}