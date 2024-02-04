use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};
use songbird::tracks::PlayMode;

use crate::utils::message::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut response_embed = CreateEmbed::default();

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
                    response_embed
                        .description("Error resuming song!")
                        .color(Color::DARK_RED);

                    respond_to_command(command, &ctx.http, response_embed).await;

                    return;
                }
            },
            None => {
                response_embed
                    .description("There is no song to resume!")
                    .color(Color::DARK_RED);

                respond_to_command(command, &ctx.http, response_embed).await;

                return;
            }
        };

        // If the song is paused, resume it
        match song_state {
            PlayMode::Pause => match current_song {
                Some(song) => match song.play() {
                    Ok(_) => {
                        response_embed
                            .description("Song **resumed!**")
                            .color(Color::DARK_GREEN);
                    }
                    Err(why) => {
                        println!("Error resuming song: {why}");
                        response_embed
                            .description("Error resuming song!")
                            .color(Color::DARK_RED);
                    }
                },
                None => {
                    response_embed
                        .description("There is nothing to resume!")
                        .color(Color::DARK_RED);
                }
            },
            _ => {
                response_embed
                    .description("The song is currently playing!")
                    .color(Color::DARK_RED);
            }
        };
    } else {
        response_embed
            .description("Error resuming song!")
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("resume")
        .description("Resume the currently paused song")
}
