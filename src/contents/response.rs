use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContent {
    pub username: String,
    pub sponser: Sponser,
    pub github: Github,
    pub twitter: Twitter,
    pub website: Website,

    #[serde(rename = "realName")]
    pub real_name: String,
    pub class: String,
    pub age: String,
    pub location: Location,
    pub birthday: Birthday,
    pub langs: String,
    pub hobbies: Hobbie,

    #[serde(rename = "welcomeMessage")]
    pub welcome_msg: String,
    #[serde(rename = "errorMessage")]
    pub error_msg: String,
    #[serde(rename = "sponserImage")]
    pub sponser_image: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Github {
    pub username: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Twitter {
    pub username: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Website {
    pub url: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sponser {
    pub url: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
}
