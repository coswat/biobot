use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Buttons {
    pub bio: String,
    pub username: String,
    pub friends: String,
    pub github: String,
    pub twitter: String,
    pub website: String,

    #[serde(rename = "realName")]
    pub real_name: String,
    pub class: String,
    pub age: String,
    pub location: String,
    pub bday: String,
    pub sigma: String,
    pub langs: String,
    pub hobbies: String,
    pub back: String,
}
