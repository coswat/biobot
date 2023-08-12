use crate::contents::{get_buttons, Buttons};
use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub async fn default() -> KeyboardMarkup {
    let button: Buttons = get_buttons().await;
    let row1: Vec<KeyboardButton> = vec![KeyboardButton::new(button.friends)];
    let row2: Vec<KeyboardButton> = vec![
        KeyboardButton::new(button.github),
        KeyboardButton::new(button.twitter),
    ];
    let row3: Vec<KeyboardButton> = vec![KeyboardButton::new(button.website)];

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
    let button: Buttons = get_buttons().await;
    let row1: Vec<KeyboardButton> = vec![
        KeyboardButton::new(button.age),
        KeyboardButton::new(button.location),
    ];
    let row2: Vec<KeyboardButton> = vec![
        KeyboardButton::new(button.bday),
        KeyboardButton::new(button.sigma),
    ];
    let row3: Vec<KeyboardButton> = vec![
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
