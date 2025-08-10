use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

use crate::utils::response::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let guild_id = command
        .guild_id
        .expect("No Guild ID found on interaction")
        .to_string();

    println!("Ping! From guild id: {guild_id}");

    respond_to_command(command, &ctx.http, format!("Pong!"), false).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Respond with Pong!")
}
