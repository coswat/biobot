use crate::command::{Command, State};
use crate::contents::ResponseContent;
use crate::keyboard;
use std::error::Error;
use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::*, types::InputFile, utils::command::BotCommands,
};
use url::Url;

pub async fn welcome(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let keyboard = keyboard::default().await;
    let sticker = InputFile::file_id(
        "CAACAgIAAxkBAAEB_x9k1l2EdUiuKNLN_guQXp8I4hjGVgACQhAAAjPFKUmQDtQRpypKgjAE".to_string(),
    );
    bot.set_my_commands(Command::bot_commands()).await?;
    bot.send_sticker(msg.chat.id, sticker).await?;
    bot.send_message(msg.chat.id, "Welcome")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn bio(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let keyboard = keyboard::bio().await;
    bot.send_message(msg.chat.id, cnt.welcome_msg)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn invalid(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, cnt.error_msg).await?;
    Ok(())
}

pub async fn username(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let message = format!("Telegram Username @{}", cnt.username);
    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}

pub async fn sponser(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let buttons = keyboard::sponser_items().await;
    let image_url = Url::parse(cnt.sponser_image.as_str()).unwrap();
    bot.send_photo(msg.chat.id, InputFile::url(image_url))
        .reply_markup(buttons)
        .await?;
    Ok(())
}

pub async fn github(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let github_link = format!("https://github.com/{}", cnt.github.username);
    let keyboard = keyboard::create_inline_url(github_link).await;
    let file_url = Url::parse(cnt.github.photo_url.as_str()).unwrap();
    bot.send_photo(msg.chat.id, InputFile::url(file_url))
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn twitter(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let twitter_link = format!("https://x.com/{}", cnt.twitter.username);
    let keyboard = keyboard::create_inline_url(twitter_link).await;
    let file_url = Url::parse(cnt.twitter.photo_url.as_str()).unwrap();
    bot.send_photo(msg.chat.id, InputFile::url(file_url))
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn website(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let keyboard = keyboard::create_inline_url(cnt.website.url).await;
    let file_url = Url::parse(cnt.website.photo_url.as_str()).unwrap();
    bot.send_photo(msg.chat.id, InputFile::url(file_url))
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn realname(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let text = format!("Real Name : {}", cnt.real_name);
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn class(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let text = format!("Class : {}", cnt.class);
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn age(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, cnt.age).await?;
    Ok(())
}

pub async fn location(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    bot.send_location(msg.chat.id, cnt.location.latitude, cnt.location.longitude)
        .await?;
    bot.send_message(msg.chat.id, cnt.location.msg).await?;
    Ok(())
}

pub async fn birthday(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let sticker = InputFile::file_id(cnt.birthday.sticker_id);
    bot.send_message(msg.chat.id, cnt.birthday.date).await?;
    bot.send_sticker(msg.chat.id, sticker).await?;
    Ok(())
}

pub async fn sigma(bot: &Bot, msg: &Message, _cnt: ResponseContent) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Yes 💀").await?;
    Ok(())
}

pub async fn langs(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, cnt.langs).await?;
    Ok(())
}

pub async fn hobbies(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let sticker = InputFile::file_id(cnt.hobbies.sticker_id);
    bot.send_message(msg.chat.id, cnt.hobbies.msg).await?;
    bot.send_sticker(msg.chat.id, sticker).await?;
    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn cancel(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    msg: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Cancelling ongoing action.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}
