use crate::contents::{Buttons, ResponseContent, Sponser};
use serde_json::from_str;
use std::env;
use std::fs;

pub async fn get_contents() -> ResponseContent {
    let mut path = env::current_dir().expect("Unable to load path");
    path.push("json/bio.json");
    let json = fs::read_to_string(path).expect("Unable to read file");

    from_str::<ResponseContent>(&json).unwrap()
}

pub async fn get_buttons() -> Buttons {
    let mut path = env::current_dir().expect("Unable to load path");
    path.push("json/buttons.json");
    let json = fs::read_to_string(path).expect("Unable to read file");

    from_str::<Buttons>(&json).unwrap()
}

pub async fn get_sponser_data() -> Sponser {
    let mut path = env::current_dir().expect("Unable to load path");
    path.push("json/sponser.json");
    let json = fs::read_to_string(path).expect("Unable to read file");

    from_str::<Sponser>(&json).unwrap()
}
