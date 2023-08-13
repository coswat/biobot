use crate::contents::get_buttons;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};
use url::Url;

pub async fn default() -> KeyboardMarkup {
    let button = get_buttons().await;
    let row1 = vec![KeyboardButton::new(button.friends)];
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
    let button = InlineKeyboardButton::url("Open Link".to_string(), url);
    InlineKeyboardMarkup::default().append_row(vec![button])
}
