use crate::contents::{Buttons, ResponseContent};
use serde_json::from_str;
use std::env;
use std::fs;

pub async fn get_contents() -> ResponseContent {
    let mut path = env::current_dir().expect("Unable to load path");
    path.push("json/bio.json");
    let json = fs::read_to_string(path).expect("Unable to read file");
    let contents = from_str::<ResponseContent>(&json).unwrap();
    contents
}

pub async fn get_buttons() -> Buttons {
    let mut path = env::current_dir().expect("Unable to load path");
    path.push("json/buttons.json");
    let json = fs::read_to_string(path).expect("Unable to read file");
    let buttons = from_str::<Buttons>(&json).unwrap();
    buttons
}
