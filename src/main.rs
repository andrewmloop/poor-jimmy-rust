mod commands;
mod handlers;
mod utils;

use std::env;

use handlers::bot_event::BotEventHandler;
use reqwest::Client as HttpClient;
use serenity::client::ClientBuilder;
use serenity::prelude::*;
use songbird::SerenityInit;
use utils::map::HttpKey;

#[tokio::main]
async fn main() {
    // DISCORD_TOKEN is required. Bot will not work without it.
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env variable was not set!");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = ClientBuilder::new(token, intents)
        .register_songbird()
        .event_handler(BotEventHandler)
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why}")
    }
}
