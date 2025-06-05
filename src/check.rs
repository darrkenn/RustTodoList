use std::fs::File;
use std::path::Path;
use crate::FILENAME;

pub fn check(){
    //Checks if toml doesnt exist, if so it is created.
    if !Path::new(FILENAME).exists() {File::create("todo.toml").unwrap();}
}