use serenity::{builder::CreateApplicationCommand, utils::Color};

use crate::utils::result::CommandResponse;

pub fn run() -> CommandResponse {
    let help_description = String::from("
    ## 🎶 Poor Jimmy Commands 🎶
    \nUse these commands to control the music playback in your server. Enjoy the tunes! 🎵
    \n### 1. /clear\nStop the current song and clear the queue
    \n### 2. /help\nDisplays this help message, providing information on available commands
    \n### 3. /join\nSummon Poor Jimmy to your voice channel
    \n### 4. /leave\nRemove Poor Jimmy from the voice channel
    \n### 5. /list\nDisplay the current queue of songs
    \n### 6. /pause\nPause the currently playing song
    \n### 7. /ping\nRespond with Pong!
    \n### 8. /play [url]\nPlay the audio from a Youtube video. This currently only supports standalone Youtube videos. Youtube playlists and Spotify funcionality comming soon!
    \n### 9. /resume\nResume the currently paused song
    \n### 10. /skip\nSkip the currently playing song");

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
