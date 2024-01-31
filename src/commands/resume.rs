use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};
use songbird::tracks::PlayMode;

use crate::utils::result::CommandResponse;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> CommandResponse {
    let response: CommandResponse;

    // Grab the voice client registered with Serentiy's shard key-value store
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    // Grab the active Call for the command's guild
    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        // Grab the currrently playing song
        let current_song = handler.queue().current();

        // Attempt to grab the current play state of the current song
        let song_state = match &current_song {
            Some(track) => match track.get_info().await {
                Ok(state) => state.playing,
                Err(why) => {
                    println!("Error getting song state: {why}");
                    response = CommandResponse::new()
                        .description(String::from("Error resuming song!"))
                        .color(Color::DARK_RED)
                        .clone();

                    return response;
                }
            },
            None => {
                response = CommandResponse::new()
                    .description(String::from("There is no song to resume!"))
                    .color(Color::DARK_RED)
                    .clone();

                return response;
            }
        };

        // If the song is paused, resume it
        match song_state {
            PlayMode::Pause => match current_song {
                Some(song) => match song.play() {
                    Ok(_) => {
                        response = CommandResponse::new()
                            .description(String::from("Song **resumed!**"))
                            .color(Color::DARK_RED)
                            .clone();
                    }
                    Err(why) => {
                        println!("Error resuming song: {why}");
                        response = CommandResponse::new()
                            .description(String::from("Error resuming song!"))
                            .color(Color::DARK_RED)
                            .clone();
                    }
                },
                None => {
                    response = CommandResponse::new()
                        .description(String::from("There is nothing to resume!"))
                        .color(Color::DARK_RED)
                        .clone();
                }
            },
            _ => {
                response = CommandResponse::new()
                    .description(String::from("There is nothing to resume!"))
                    .color(Color::DARK_RED)
                    .clone();
            }
        };
    } else {
        response = CommandResponse::new()
            .description(String::from("Error resuming song!"))
            .color(Color::DARK_RED)
            .clone();
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("resume")
        .description("Resume the currently paused song")
}
