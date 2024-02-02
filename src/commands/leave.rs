use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

use crate::utils::result::CommandResponse;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> CommandResponse {
    /*
       1. Grab guild_id
       2. Grab the songbird manager
       3. Use the manager to leave the voice channel
       4. Return a "Poor Jimmy has left the voice chat" on success
       5. Return a "Error leaving the voice channel" for all other errors"
    */
    let response: CommandResponse;

    let guild_id = command.guild_id.unwrap();

    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    if let Ok(_) = manager.leave(guild_id).await {
        response = CommandResponse::new()
            .description(String::from("Poor Jimmy left the voice channel"))
            .color(Color::DARK_GREEN)
            .clone();
    } else {
        response = CommandResponse::new()
            .description(String::from("Error leaving voice channel"))
            .color(Color::DARK_RED)
            .clone();

        return response;
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .description("Remove Poor Jimmy from the voice channel")
}
