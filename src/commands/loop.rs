use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};
use songbird::tracks::LoopState;

use crate::utils::response::respond_to_command;

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

        // Grab the state of the current song
        let is_looping = match &current_song {
            Some(track) => {
                match track.get_info().await {
                    Ok(state) => state.loops.eq(&LoopState::Infinite),
                    // If we can't get the track's state, return early
                    Err(why) => {
                        println!("Error getting song state: {why}");
                        response_embed
                            .description("Error looping song!")
                            .color(Color::DARK_RED);

                        respond_to_command(command, &ctx.http, response_embed).await;

                        return;
                    }
                }
            }
            // If the queue is empty, return early
            None => {
                response_embed
                    .description("There is no song to pause!")
                    .color(Color::DARK_RED);

                respond_to_command(command, &ctx.http, response_embed).await;

                return;
            }
        };

        if is_looping {
            match current_song.unwrap().disable_loop() {
                Ok(_) => {
                    response_embed
                        .description("Disabled **looping!**")
                        .color(Color::DARK_GREEN);
                }
                // Error disabling loop, return early
                Err(why) => {
                    println!("Error disabling looping: {why}");
                    response_embed
                        .description("Error looping song!")
                        .color(Color::DARK_RED);

                    respond_to_command(command, &ctx.http, response_embed).await;

                    return;
                }
            }
        } else {
            match current_song.unwrap().enable_loop() {
                Ok(_) => {
                    response_embed
                        .description("Enabled **looping!** Use **/loop** again to disable or **/skip** to skip")
                        .color(Color::DARK_GREEN);
                }
                // Error enabling loop, return early
                Err(why) => {
                    println!("Error looping song: {why}");
                    response_embed
                        .description("Error looping song!")
                        .color(Color::DARK_RED);

                    respond_to_command(command, &ctx.http, response_embed).await;

                    return;
                }
            }
        }
    } else {
        response_embed
            .description(
                "Error looping song! Ensure Poor Jimmy is in a voice channel with **/join**",
            )
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("loop")
        .description("Enable/disable looping for the current song")
}
