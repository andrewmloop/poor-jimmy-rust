use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::message_component::MessageComponentInteraction,
    },
};

use crate::utils::response::{
    respond_to_button, respond_to_command, respond_to_error, respond_to_error_button,
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        let queue_length = handler.queue().len();

        if queue_length == 0 {
            respond_to_command(
                command,
                &ctx.http,
                format!("There is nothing to clear!"),
                false,
            )
            .await;
        } else {
            handler.queue().stop();

            respond_to_command(command, &ctx.http, format!("Queue **cleared!**"), false).await;
        }
    } else {
        respond_to_error(
            command,
            &ctx.http,
            format!("Error clearing queue! Ensure Poor Jimmy is in a voice channel with **/join**"),
        )
        .await;
    }
}

pub async fn handle_button(ctx: &Context, command: &MessageComponentInteraction) {
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        let queue_length = handler.queue().len();

        if queue_length == 0 {
            respond_to_button(
                command,
                &ctx.http,
                format!("There is nothing to clear!"),
                false,
            )
            .await;
        } else {
            handler.queue().stop();

            respond_to_button(command, &ctx.http, format!("Queue **cleared!**"), false).await;
        }
    } else {
        respond_to_error_button(
            command,
            &ctx.http,
            format!("Error clearing queue! Ensure Poor Jimmy is in a voice channel with **/join**"),
        )
        .await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("clear")
        .description("Stop the current song and clear the queue")
}
