use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

use crate::utils::result::CommandResponse;

pub fn run(command: &ApplicationCommandInteraction) -> CommandResponse {
    let guild_id = command
        .guild_id
        .expect("No Guild ID found on interaction")
        .to_string();

    println!("Ping! From guild id: {guild_id}");

    CommandResponse::new()
        .description(String::from("Pong!"))
        .color(Color::DARK_GREEN)
        .clone()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Responds with Pong!")
}
