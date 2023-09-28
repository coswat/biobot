use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sponser {
    pub enabled: bool,
    pub currency: String,
    pub item1: Item,
    pub item2: Item,
    pub item3: Item,
    pub item4: Item,
    pub item5: Item,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}
