use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

use crate::utils::response::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut response_embed = CreateEmbed::default();

    let guild_id = command
        .guild_id
        .expect("No Guild ID found on interaction")
        .to_string();

    println!("Ping! From guild id: {guild_id}");

    response_embed.description("Pong!").color(Color::DARK_GREEN);

    respond_to_command(command, &ctx.http, response_embed).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Respond with Pong!")
}
