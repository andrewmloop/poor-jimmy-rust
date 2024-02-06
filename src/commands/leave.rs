use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

use crate::utils::message::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    /*
       1. Grab guild_id
       2. Grab the songbird manager
       3. Use the manager to leave the voice channel
       4. Return a "Poor Jimmy has left the voice chat" on success
       5. Return a "Error leaving the voice channel" for all other errors"
    */
    let mut response_embed = CreateEmbed::default();

    let guild_id = command.guild_id.unwrap();

    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    if let Ok(_) = manager.leave(guild_id).await {
        response_embed
            .description("Poor Jimmy **left** the voice channel!")
            .color(Color::DARK_GREEN);
    } else {
        response_embed
            .description("Error leaving voice channel! Ensure Poor Jimmy is in a voice channel with **/join**")
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .description("Remove Poor Jimmy from the voice channel")
}
