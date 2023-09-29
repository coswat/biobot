use crate::{
    bio,
    contents::{get_buttons, get_contents},
};
use std::error::Error;
use teloxide::{requests::Requester, types::Message, Bot};

pub async fn init(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = &msg.text() {
        route(text, bot, &msg).await?;
    } else {
        bot.send_message(msg.chat.id, "Invalid message type")
            .await?;
    }
    Ok(())
}

async fn route(text: &str, bot: Bot, msg: &Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let contents = get_contents().await;
    let button = get_buttons().await;
    match text {
        _ if button.bio == text => bio::bio(&bot, msg, contents).await?,
        _ if button.back == text => bio::welcome(bot, msg).await?,
        _ if button.username == text => bio::username(&bot, msg, contents).await?,
        _ if button.sponser == text => bio::sponser(&bot, msg, contents).await?,
        _ if button.github == text => bio::github(&bot, msg, contents).await?,
        _ if button.twitter == text => bio::twitter(&bot, msg, contents).await?,
        _ if button.website == text => bio::website(&bot, msg, contents).await?,
        _ if button.real_name == text => bio::realname(&bot, msg, contents).await?,
        _ if button.class == text => bio::class(&bot, msg, contents).await?,
        _ if button.age == text => bio::age(&bot, msg, contents).await?,
        _ if button.location == text => bio::location(&bot, msg, contents).await?,
        _ if button.bday == text => bio::birthday(&bot, msg, contents).await?,
        _ if button.sigma == text => bio::sigma(&bot, msg, contents).await?,
        _ if button.langs == text => bio::langs(&bot, msg, contents).await?,
        _ if button.hobbies == text => bio::hobbies(&bot, msg, contents).await?,
        _ => bio::invalid(&bot, msg, contents).await?,
    }
    Ok(())
}
