use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

use crate::utils::response::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let help_description = String::from(
        "
    ## 🎶 Poor Jimmy Commands 🎶
    \nUse these commands to control the music playback in your server. Enjoy the tunes! 🎵
    \n**1. /clear**Stop the current song and clear the queue
    \n**2. /help**Displays this help message, providing information on available commands
    \n**3. /join**Summon Poor Jimmy to your voice channel
    \n**4. /leave**Remove Poor Jimmy from the voice channel
    \n**5. /list**Display the current queue of songs
    \n**6. /loop**Enable/disable looping of the current song
    \n**7. /pause**Pause the currently playing song
    \n**8. /ping**Respond with Pong!
    \n**9. /play-url**Play the audio from a Youtube video or playlist URL
    \n**10. /play-title**Play the audio from a Youtube video best matching the given title
    \n**11. /resume**Resume the currently paused song
    \n**12. /skip**Skip the currently playing song",
    );

    respond_to_command(command, &ctx.http, help_description, false).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Display directions on how to use Poor Jimmy's commands")
}
