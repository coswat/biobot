use crate::bio;
use crate::contents::{get_buttons, get_contents, Buttons, ResponseContent};
use teloxide::{requests::ResponseResult, types::Message, Bot};

pub async fn init(bot: Bot, msg: Message) -> ResponseResult<()> {
    let contents: ResponseContent = get_contents().await;
    let button: Buttons = get_buttons().await;
    let text = &msg.text().unwrap().to_string();
    if "/start" == *text {
        bio::welcome(&bot, &msg, contents).await?;
    } else if button.bio == *text {
        bio::bio(&bot, &msg, contents).await?;
    } else if button.back == *text {
        bio::welcome(&bot, &msg, contents).await?;
    } else if button.username == *text {
        bio::username(&bot, &msg, contents).await?;
    } else if button.friends == *text {
        bio::friends(&bot, &msg, contents).await?;
    } else if button.github == *text {
        bio::github(&bot, &msg, contents).await?;
    } else if button.twitter == *text {
        bio::twitter(&bot, &msg, contents).await?;
    } else if button.website == *text {
        bio::website(&bot, &msg, contents).await?;
    } else if button.real_name == *text {
        bio::realname(&bot, &msg, contents).await?;
    } else if button.class == *text {
        bio::class(&bot, &msg, contents).await?;
    } else if button.age == *text {
        bio::age(&bot, &msg, contents).await?;
    } else if button.location == *text {
        bio::location(&bot, &msg, contents).await?;
    } else if button.bday == *text {
        bio::birthday(&bot, &msg, contents).await?;
    } else if button.sigma == *text {
        bio::sigma(&bot, &msg, contents).await?;
    } else if button.langs == *text {
        bio::langs(&bot, &msg, contents).await?;
    } else if button.hobbies == *text {
        bio::hobbies(&bot, &msg, contents).await?;
    } else {
        bio::invalid(&bot, &msg, contents).await?;
    }
    Ok(())
}
