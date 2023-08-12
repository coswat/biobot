use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContent {
    pub username: String,
    pub friends: String,
    pub github: String,
    pub twitter: String,
    pub website: String,

    #[serde(rename = "realName")]
    pub real_name: String,
    pub class: String,
    pub age: u8,
    pub location: Location,
    pub birthday: Birthday,
    pub langs: String,
    pub hobbies: Hobbie,

    #[serde(rename = "welcomeMessage")]
    pub welcome_msg: String,
    #[serde(rename = "errorMessage")]
    pub error_msg: String,
    #[serde(rename = "welcomeSticker")]
    pub welcome_sticker: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hobbie {
    pub msg: String,
    #[serde(rename = "stickerId")]
    pub sticker_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Birthday {
    pub date: String,
    #[serde(rename = "stickerId")]
    pub sticker_id: String,
}
