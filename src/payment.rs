use crate::contents::{get_sponser_data, Item};
use std::env;
use std::error::Error;
use teloxide::{
    payloads::{AnswerPreCheckoutQuerySetters, SendInvoiceSetters},
    prelude::Requester,
    types::{CallbackQuery, ChatId, LabeledPrice, PreCheckoutQuery},
    Bot,
};
use url::Url;

pub async fn pre_checkout_handler(
    bot: Bot,
    query: PreCheckoutQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.answer_pre_checkout_query(query.id, false)
        .error_message("Sponser me in Github instead")
        .await?;
    Ok(())
}

pub async fn callback_handler(
    bot: Bot,
    query: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = query.data.unwrap();
    let items = get_sponser_data().await;
    let currency = items.currency;
    let msg = query.message.unwrap();

    match data.as_str() {
        "item1" => self::invoice(bot, msg.chat.id, items.item1, currency).await?,
        "item2" => self::invoice(bot, msg.chat.id, items.item2, currency).await?,
        "item3" => self::invoice(bot, msg.chat.id, items.item3, currency).await?,
        "item4" => self::invoice(bot, msg.chat.id, items.item4, currency).await?,
        _ => self::invoice(bot, msg.chat.id, items.item5, currency).await?,
    }

    Ok(())
}

pub async fn invoice(
    bot: Bot,
    id: ChatId,
    item: Item,
    currency: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let photo = Url::parse(item.image_url.as_str())?;
    let token = env::var("PAYMENT_TOKEN")?;
    let price = [LabeledPrice {
        label: String::from("amount"),
        amount: item.price * 100,
    }];
    bot.send_invoice(
        id,
        item.name,
        item.description,
        "payload-custom",
        token,
        currency,
        price.into_iter(),
    )
    .max_tip_amount(2000)
    .suggested_tip_amounts(vec![100, 200, 500, 1000])
    .photo_url(photo)
    .photo_width(String::from("120"))
    .photo_width(String::from("120"))
    .await?;
    Ok(())
}
