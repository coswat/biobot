use teloxide::utils::command::BotCommands;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display all commands")]
    Help,
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "cancel ongoing action")]
    Cancel,
}
