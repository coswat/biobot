use crate::contents::{get_buttons, get_sponser_data};
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, WebAppInfo,
};
use url::Url;

pub async fn default() -> KeyboardMarkup {
    let button = get_buttons().await;
    let row1 = vec![KeyboardButton::new(button.sponser)];
    let row2 = vec![
        KeyboardButton::new(button.github),
        KeyboardButton::new(button.twitter),
    ];
    let row3 = vec![KeyboardButton::new(button.website)];

    KeyboardMarkup::new(vec![vec![
        KeyboardButton::new(button.bio),
        KeyboardButton::new(button.username),
    ]])
    .append_row(row1)
    .append_row(row2)
    .append_row(row3)
    .resize_keyboard(true)
}

pub async fn bio() -> KeyboardMarkup {
    let button = get_buttons().await;
    let row1 = vec![
        KeyboardButton::new(button.age),
        KeyboardButton::new(button.location),
    ];
    let row2 = vec![
        KeyboardButton::new(button.bday),
        KeyboardButton::new(button.sigma),
    ];
    let row3 = vec![
        KeyboardButton::new(button.langs),
        KeyboardButton::new(button.hobbies),
        KeyboardButton::new(button.back),
    ];
    KeyboardMarkup::new(vec![vec![
        KeyboardButton::new(button.real_name),
        KeyboardButton::new(button.class),
    ]])
    .append_row(row1)
    .append_row(row2)
    .append_row(row3)
    .resize_keyboard(true)
}

pub async fn create_inline_url(link: String) -> InlineKeyboardMarkup {
    let url = Url::parse(link.as_str()).unwrap();
    let button = InlineKeyboardButton::url("Open Link", url.clone());
    let button2 = InlineKeyboardButton::web_app("View Inline", WebAppInfo { url: url.clone() });
    InlineKeyboardMarkup::default()
        .append_row(vec![button])
        .append_row(vec![button2])
}

pub async fn sponser_items() -> InlineKeyboardMarkup {
    let sponser = get_sponser_data().await;
    let button = vec![
        InlineKeyboardButton::callback(sponser.item1.name, "item1"),
        InlineKeyboardButton::callback(sponser.item2.name, "item2"),
    ];
    let button2 = vec![
        InlineKeyboardButton::callback(sponser.item3.name, "item3"),
        InlineKeyboardButton::callback(sponser.item4.name, "item4"),
    ];
    let button3 = vec![InlineKeyboardButton::callback(sponser.item5.name, "item5")];
    InlineKeyboardMarkup::default()
        .append_row(button)
        .append_row(button2)
        .append_row(button3)
}
