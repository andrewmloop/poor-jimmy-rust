use serenity::{builder::CreateApplicationCommand, utils::Color};

use crate::utils::result::CommandResponse;

pub fn run() -> CommandResponse {
    let help_description = String::from("
    ## 🎶 Poor Jimmy Commands 🎶
    \nUse these commands to control the music playback in your server. Enjoy the tunes! 🎵
    \n**1. /clear**Stop the current song and clear the queue
    \n**2. /help**Displays this help message, providing information on available commands
    \n**3. /join**Summon Poor Jimmy to your voice channel
    \n**4. /leave**Remove Poor Jimmy from the voice channel
    \n**5. /list**Display the current queue of songs
    \n**6. /pause**Pause the currently playing song
    \n**7. /ping**Respond with Pong!
    \n**8. /play [url]**Play the audio from a Youtube video or playlist. Spotify funcionality coming soon!
    \n**9. /resume**Resume the currently paused song
    \n**10. /skip**Skip the currently playing song");

    CommandResponse::new()
        .description(help_description)
        .color(Color::DARK_GREEN)
        .clone()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Display directions on how to use Poor Jimmy's commands")
}
