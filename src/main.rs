mod bio;
mod bot;
mod command;
mod contents;
mod handler;
mod keyboard;
mod payment;

#[tokio::main]
async fn main() {
    use bot;
    bot::run().await;
}
