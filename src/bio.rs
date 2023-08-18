use crate::contents::ResponseContent;
use crate::keyboard;
use teloxide::prelude::*;
use teloxide::types::{InputFile, ParseMode};

pub async fn welcome(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let keyboard = keyboard::default().await;
    let sticker = InputFile::file_id(cnt.welcome_sticker);
    bot.send_sticker(msg.chat.id, sticker).await?;
    bot.send_message(msg.chat.id, "Welcome")
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn bio(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let keyboard = keyboard::bio().await;
    bot.send_message(msg.chat.id, cnt.welcome_msg)
        .parse_mode(ParseMode::Html)
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

pub async fn friends(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, cnt.friends).await?;
    Ok(())
}

pub async fn github(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let github_link = format!("https://github.com/{}", cnt.github.username);
    let keyboard = keyboard::create_inline_url(github_link).await;
    bot.send_message(msg.chat.id, cnt.github.msg)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn twitter(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let twitter_link = format!("https://x.com/{}", cnt.twitter.username);
    let keyboard = keyboard::create_inline_url(twitter_link).await;
    bot.send_message(msg.chat.id, cnt.twitter.msg)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn website(bot: &Bot, msg: &Message, cnt: ResponseContent) -> ResponseResult<()> {
    let keyboard = keyboard::create_inline_url(cnt.website.url).await;
    bot.send_message(msg.chat.id, cnt.website.msg)
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
