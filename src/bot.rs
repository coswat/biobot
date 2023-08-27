use crate::bio;
use crate::command::{Command, State};
use crate::handler as bothandler;
use crate::payment;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    update_listeners::webhooks,
};

pub async fn run() {
    pretty_env_logger::init();
    log::info!("Starting bio bot...");
    dotenv().ok();

    let bot: Bot = Bot::from_env();
    let dev_mod = env::var("DEV")
        .expect("env var DEV not set")
        .parse::<bool>()
        .unwrap();
    if dev_mod {
        Dispatcher::builder(bot, schema())
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    } else {
        let port: u16 = env::var("PORT")
            .expect("PORT env variable is not set")
            .parse()
            .expect("PORT env variable value is not an integer");

        let addr = ([0, 0, 0, 0], port).into();

        let host = env::var("HOST").expect("HOST env variable is not set");
        let url = format!("https://{host}/webhook").parse().unwrap();

        let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
            .await
            .expect("Couldn't setup webhook");
        Dispatcher::builder(bot, schema())
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch_with_listener(
                listener,
                LoggingErrorHandler::with_custom_text("An error has occurred in the dispatcher"),
            )
            .await;
    }
}

fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    use dptree::case;
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(bio::help))
                .branch(case![Command::Start].endpoint(bio::welcome)),
        )
        .branch(case![Command::Cancel].endpoint(bio::cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(bothandler::init));
    dptree::entry()
        .branch(
            dialogue::enter::<Update, InMemStorage<_>, State, _>()
                .branch(message_handler)
                .branch(Update::filter_callback_query().endpoint(payment::callback_handler)),
        )
        .branch(Update::filter_pre_checkout_query().endpoint(payment::pre_checkout_handler))
}
