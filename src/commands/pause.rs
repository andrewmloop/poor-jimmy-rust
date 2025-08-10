use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::message_component::MessageComponentInteraction,
    },
};
use songbird::tracks::PlayMode;

use crate::utils::response::{
    respond_to_button, respond_to_command, respond_to_error, respond_to_error_button,
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        let current_song = handler.queue().current();

        // Attempt to grab the current play state of the current song
        let song_state = match &current_song {
            Some(track) => match track.get_info().await {
                Ok(state) => state.playing,
                Err(why) => {
                    println!("Error getting song state: {why}");

                    respond_to_error(command, &ctx.http, format!("Error pausing song!")).await;

                    return;
                }
            },
            None => {
                respond_to_command(
                    command,
                    &ctx.http,
                    format!("There is no song to pause!"),
                    false,
                )
                .await;

                return;
            }
        };

        // If the song is playing, pause it
        match song_state {
            PlayMode::Play => match current_song {
                Some(song) => match song.pause() {
                    Ok(_) => {
                        respond_to_command(
                            command,
                            &ctx.http,
                            format!("Song **paused!** Use **/resume** to continue playback"),
                            true,
                        )
                        .await;
                    }
                    Err(why) => {
                        println!("Error resuming song: {why}");

                        respond_to_error(command, &ctx.http, format!("Error pausing song!")).await;
                    }
                },
                None => {
                    respond_to_command(
                        command,
                        &ctx.http,
                        format!("There is nothing to pause!"),
                        false,
                    )
                    .await;
                }
            },
            _ => {
                respond_to_command(
                    command,
                    &ctx.http,
                    format!("The song is currently paused!"),
                    true,
                )
                .await;
            }
        };
    } else {
        respond_to_error(
            command,
            &ctx.http,
            format!("Error pausing song! Ensure Poor Jimmy is in a voice channel with **/join**"),
        )
        .await;
    }
}

pub async fn handle_button(ctx: &Context, command: &MessageComponentInteraction) {
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        let current_song = handler.queue().current();

        // Attempt to grab the current play state of the current song
        let song_state = match &current_song {
            Some(track) => match track.get_info().await {
                Ok(state) => state.playing,
                Err(why) => {
                    println!("Error getting song state: {why}");

                    respond_to_error_button(command, &ctx.http, format!("Error pausing song!"))
                        .await;

                    return;
                }
            },
            None => {
                respond_to_button(
                    command,
                    &ctx.http,
                    format!("There is no song to pausse!"),
                    false,
                )
                .await;

                return;
            }
        };

        // If the song is playing, pause it
        match song_state {
            PlayMode::Play => match current_song {
                Some(song) => match song.pause() {
                    Ok(_) => {
                        respond_to_button(
                            command,
                            &ctx.http,
                            format!("Song **paused!** Use **/resume** to continue playback"),
                            true,
                        )
                        .await;
                    }
                    Err(why) => {
                        println!("Error resuming song: {why}");

                        respond_to_error_button(command, &ctx.http, format!("Error pausing song!"))
                            .await;
                    }
                },
                None => {
                    respond_to_button(
                        command,
                        &ctx.http,
                        format!("There is nothing to pause!"),
                        false,
                    )
                    .await;
                }
            },
            _ => {
                respond_to_button(
                    command,
                    &ctx.http,
                    format!("The song is currently paused!"),
                    true,
                )
                .await;
            }
        };
    } else {
        respond_to_error_button(
            command,
            &ctx.http,
            format!("Error pausing song! Ensure Poor Jimmy is in a voice channel with **/join**"),
        )
        .await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("pause")
        .description("Pause the currently playing song")
}
