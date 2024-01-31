mod commands;
mod utils;

use std::env;

use crate::utils::map::HttpKey;
use crate::utils::result::CommandResponse;

use reqwest::Client as HttpClient;
use serenity::async_trait;
use serenity::client::{ClientBuilder, Context, EventHandler};
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::Color;
use songbird::SerenityInit;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_name = command.data.name.as_str();
            println!("Received command interaction: {command_name:#?}");

            let response = match command_name {
                "join" => Some(commands::join::run(&ctx, &command).await),
                "leave" => Some(commands::leave::run(&ctx, &command).await),
                "pause" => Some(commands::pause::run(&ctx, &command).await),
                "ping" => Some(commands::ping::run(&command)),
                "play" => Some(commands::play::run(&ctx, &command).await),
                "skip" => Some(commands::skip::run(&ctx, &command).await),
                "resume" => Some(commands::skip::run(&ctx, &command).await),
                _ => {
                    let response = CommandResponse::new()
                        .description(String::from("Unknown command!"))
                        .color(Color::DARK_RED)
                        .clone();

                    Some(response)
                } // Not Implemented
            };

            if let Some(response) = response {
                if let Err(why) = command
                    .create_interaction_response(ctx.http, |interaction_response| {
                        interaction_response.interaction_response_data(|data| {
                            data.embed(|embed| {
                                embed
                                    .description(response.get_description())
                                    .color(response.get_color())
                            })
                        })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {why}");
                };
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|c| commands::join::register(c))
                .create_application_command(|c| commands::leave::register(c))
                .create_application_command(|c| commands::pause::register(c))
                .create_application_command(|c| commands::ping::register(c))
                .create_application_command(|c| commands::play::register(c))
                .create_application_command(|c| commands::skip::register(c))
                .create_application_command(|c| commands::resume::register(c))
        })
        .await
        .expect("Failed to register slash commands!");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file!");

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env variable was not set!");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = ClientBuilder::new(token, intents)
        .register_songbird()
        .event_handler(Handler)
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why}")
    }
}
