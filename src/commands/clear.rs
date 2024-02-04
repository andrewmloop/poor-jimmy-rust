use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

use crate::utils::message::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut response_embed = CreateEmbed::default();

    // Grab the voice client registered with Serentiy's shard key-value store
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    // Grab the active Call for the command's guild
    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        let queue_length = handler.queue().len();

        if queue_length == 0 {
            response_embed
                .description("There is nothing to clear!")
                .color(Color::DARK_GREEN);

            respond_to_command(command, &ctx.http, response_embed).await;

            return;
        }

        handler.queue().stop();

        response_embed
            .description("Queue **cleared!**")
            .color(Color::DARK_GREEN);
    } else {
        response_embed
            .description("Error clearing queue!")
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("clear")
        .description("Stop the current song and clear the queue")
}
