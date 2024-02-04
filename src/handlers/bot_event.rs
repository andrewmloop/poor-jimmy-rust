use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::{Context, EventHandler};
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::{Activity, Ready};
use serenity::utils::Color;

use crate::commands;
use crate::utils::message::respond_to_command;

/// The primary handler for the bot that handles all
/// the events for the client
pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_name = command.data.name.as_str();

            match command_name {
                "clear" => commands::clear::run(&ctx, &command).await,
                "help" => commands::help::run(&ctx, &command).await,
                "join" => commands::join::run(&ctx, &command).await,
                "leave" => commands::leave::run(&ctx, &command).await,
                "list" => commands::list::run(&ctx, &command).await,
                "pause" => commands::pause::run(&ctx, &command).await,
                "ping" => commands::ping::run(&ctx, &command).await,
                "play" => commands::play::run(&ctx, &command).await,
                "skip" => commands::skip::run(&ctx, &command).await,
                "resume" => commands::resume::run(&ctx, &command).await,
                _ => {
                    let mut response_embed = CreateEmbed::default();
                    response_embed
                        .description("Unknown command!")
                        .color(Color::DARK_RED);

                    respond_to_command(&command, &ctx.http, response_embed).await;
                }
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|c| commands::clear::register(c))
                .create_application_command(|c| commands::help::register(c))
                .create_application_command(|c| commands::join::register(c))
                .create_application_command(|c| commands::leave::register(c))
                .create_application_command(|c| commands::list::register(c))
                .create_application_command(|c| commands::pause::register(c))
                .create_application_command(|c| commands::ping::register(c))
                .create_application_command(|c| commands::play::register(c))
                .create_application_command(|c| commands::resume::register(c))
                .create_application_command(|c| commands::skip::register(c))
        })
        .await
        .expect("Failed to register slash commands!");

        ctx.set_activity(Activity::listening("/play")).await;
    }
}
