use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::{Activity, Ready};
use serenity::utils::Color;

use crate::commands;
use crate::utils::result::CommandResponse;

/// The primary handler for the bot that handles all
/// the events for the client
pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_name = command.data.name.as_str();

            let response = match command_name {
                "clear" => Some(commands::clear::run(&ctx, &command).await),
                "help" => Some(commands::help::run()),
                "join" => Some(commands::join::run(&ctx, &command).await),
                "leave" => Some(commands::leave::run(&ctx, &command).await),
                "list" => Some(commands::list::run(&ctx, &command).await),
                "pause" => Some(commands::pause::run(&ctx, &command).await),
                "ping" => Some(commands::ping::run(&command)),
                // The play command can take a long time to response depending
                // on if a playlist is being queued. The command itself
                // handles responding to the command.
                //
                // TODO: Refactor bot commands to respond to slash commands,
                //       not the handler.
                "play" => {
                    commands::play::run(&ctx, &command).await;
                    None
                }
                "skip" => Some(commands::skip::run(&ctx, &command).await),
                "resume" => Some(commands::resume::run(&ctx, &command).await),
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
