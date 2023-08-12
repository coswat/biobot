mod bio;
mod contents;
mod handler;
mod keyboard;

use crate::handler as bothandler;
use dotenv::dotenv;
use teloxide::Bot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bio bot...");
    dotenv().ok();

    let bot: Bot = Bot::from_env();

    teloxide::repl(bot, bothandler::init).await;
}
