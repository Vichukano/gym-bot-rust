use std::fs;

use log::LevelFilter;
use simple_logger::SimpleLogger;
mod bot;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    log::info!("Starting bot...");
    let token = fs::read_to_string(".token").unwrap();
    let bot = bot::gym_bot::Bot::new(token);
    let handler = bot::handler::MessageHandler::new();
    bot.start(handler);
    log::info!("Finish bot...");
}
