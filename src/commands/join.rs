use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

use crate::utils::result::CommandResponse;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> CommandResponse {
    /*
       1. Grab guild_id and voice_channel_id from context/command
       2. Grab the songbird manager
       3. Use the manager to connect to the voice channel
       4. Return a "Poor Jimmy has joined the voice chat" on success
       5. Return a "You're not in a voice channel, if one cannot be found"
       6. Return a "Error joining the voice channel for all other errors"
    */
    let response: CommandResponse;

    let guild_id = command.guild_id.unwrap();
    let user_id = {
        let member = command.member.as_ref().unwrap();
        member.user.id
    };

    let voice_channel_id = {
        let guild = match ctx.cache.guild(guild_id) {
            Some(guild) => guild,
            None => {
                println!("Error finding guild in cache");
                println!("{:?}", ctx.cache);
                response = CommandResponse::new()
                    .description("Error joining voice channel")
                    .color(Color::DARK_RED)
                    .clone();

                return response;
            }
        };
        let voice_state_channel = guild
            .voice_states
            .get(&user_id)
            .and_then(|voice_state| voice_state.channel_id);

        voice_state_channel
    };

    let connect_to = match voice_channel_id {
        Some(channel) => channel,
        None => {
            response = CommandResponse::new()
                .description("You're not in a voice channel!")
                .color(Color::DARK_RED)
                .clone();

            return response;
        }
    };

    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    let (_call, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        response = CommandResponse::new()
            .description("Poor Jimmy joined the voice channel")
            .color(Color::DARK_GREEN)
            .clone();
    } else {
        response = CommandResponse::new()
            .description("Error joining voice channel")
            .color(Color::DARK_RED)
            .clone();

        return response;
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .description("Adds Poor Jimmy to the voice channel you're in")
}
