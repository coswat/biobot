use crate::bio;
use crate::contents::{get_buttons, get_contents, Buttons, ResponseContent};
use teloxide::{requests::ResponseResult, types::Message, Bot};

pub async fn init(bot: Bot, msg: Message) -> ResponseResult<()> {
    let contents: ResponseContent = get_contents().await;
    let button: Buttons = get_buttons().await;
    let text = &msg.text().unwrap().to_string();
    match text.as_str() {
        "/start" => bio::welcome(&bot, &msg, contents).await?,
        _ if button.bio == *text => bio::bio(&bot, &msg, contents).await?,
        _ if button.back == *text => bio::welcome(&bot, &msg, contents).await?,
        _ if button.username == *text => bio::username(&bot, &msg, contents).await?,
        _ if button.friends == *text => bio::friends(&bot, &msg, contents).await?,
        _ if button.github == *text => bio::github(&bot, &msg, contents).await?,
        _ if button.twitter == *text => bio::twitter(&bot, &msg, contents).await?,
        _ if button.website == *text => bio::website(&bot, &msg, contents).await?,
        _ if button.real_name == *text => bio::realname(&bot, &msg, contents).await?,
        _ if button.class == *text => bio::class(&bot, &msg, contents).await?,
        _ if button.age == *text => bio::age(&bot, &msg, contents).await?,
        _ if button.location == *text => bio::location(&bot, &msg, contents).await?,
        _ if button.bday == *text => bio::birthday(&bot, &msg, contents).await?,
        _ if button.sigma == *text => bio::sigma(&bot, &msg, contents).await?,
        _ if button.langs == *text => bio::langs(&bot, &msg, contents).await?,
        _ if button.hobbies == *text => bio::hobbies(&bot, &msg, contents).await?,
        _ => bio::invalid(&bot, &msg, contents).await?,
    }
    Ok(())
}
