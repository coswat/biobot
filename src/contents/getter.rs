use crate::contents::{Buttons, ResponseContent};
use serde_json::from_str;
use std::env;
use std::fs;
use std::path::PathBuf;

pub async fn get_contents() -> ResponseContent {
    let mut path: PathBuf = env::current_dir().expect("Unable to load dir path");
    path.push("json/bio.json");
    let json = fs::read_to_string(path).expect("Unable to read file");
    let contents: ResponseContent = from_str(&json).unwrap();
    contents
}

pub async fn get_buttons() -> Buttons {
    let mut path: PathBuf = env::current_dir().expect("Unable to load dir path");
    path.push("json/buttons.json");
    let json = fs::read_to_string(path).expect("Unable to read file");
    let buttons: Buttons = from_str(&json).unwrap();
    buttons
}
