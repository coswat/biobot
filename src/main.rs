mod bio;
mod contents;
mod handler;
mod keyboard;

use crate::handler as bothandler;
use dotenv::dotenv;
use teloxide::Bot;

// #[tokio::main]
// async fn main() {
//     pretty_env_logger::init();
//     log::info!("Starting bio bot...");
//     dotenv().ok();
//
//     let bot: Bot = Bot::from_env();
//
//     let port: u16 = env::var("PORT")
//         .expect("PORT env variable is not set")
//         .parse()
//         .expect("PORT env variable value is not an integer");
//
//     let addr = ([0, 0, 0, 0], port).into();
//
//     let host = env::var("HOST").expect("HOST env variable is not set");
//     let url = format!("https://{host}/webhook").parse().unwrap();
//
//     let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
//         .await
//         .expect("Couldn't setup webhook");
//
//     teloxide::repl_with_listener(bot, bothandler::init, listener).await;
// }

//FOR DEVELOPMENT USE THIS MAIN FUNCTION INSTEAD OF THE ABOVE ONE

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bio bot...");
    dotenv().ok();

    let bot: Bot = Bot::from_env();

    teloxide::repl(bot, bothandler::init).await;
}
