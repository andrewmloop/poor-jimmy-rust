use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::message_component::MessageComponentInteraction,
    },
};
use songbird::tracks::LoopState;

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

                        respond_to_error(command, &ctx.http, format!("Error looping song!")).await;

                        return;
                    }
                }
            }
            // If the queue is empty, return early
            None => {
                respond_to_command(
                    command,
                    &ctx.http,
                    format!("There is no song to loop!"),
                    false,
                )
                .await;

                return;
            }
        };

        if is_looping {
            match current_song.unwrap().disable_loop() {
                Ok(_) => {
                    respond_to_command(command, &ctx.http, format!("Disabled **looping!**"), true)
                        .await;
                }
                // Error disabling loop, return early
                Err(why) => {
                    println!("Error disabling looping: {why}");

                    respond_to_error(command, &ctx.http, format!("Error looping song!")).await;
                }
            }
        } else {
            match current_song.unwrap().enable_loop() {
                Ok(_) => {
                    respond_to_command(command, &ctx.http, format!("Enabled **looping!** Use **/loop** again to disable or **/skip** to skip"), true).await;
                }
                // Error enabling loop, return early
                Err(why) => {
                    println!("Error looping song: {why}");

                    respond_to_error(command, &ctx.http, format!("Error looping song!")).await;
                }
            }
        }
    } else {
        respond_to_error(
            command,
            &ctx.http,
            format!("Error looping song! Ensure Poor Jimmy is in a voice channel with **/join**"),
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

                        respond_to_error_button(command, &ctx.http, format!("Error looping song!"))
                            .await;

                        return;
                    }
                }
            }
            // If the queue is empty, return early
            None => {
                respond_to_button(
                    command,
                    &ctx.http,
                    format!("There is no song to loop!"),
                    false,
                )
                .await;

                return;
            }
        };

        if is_looping {
            match current_song.unwrap().disable_loop() {
                Ok(_) => {
                    respond_to_button(command, &ctx.http, format!("Disabled **looping!**"), true)
                        .await;
                }
                // Error disabling loop, return early
                Err(why) => {
                    println!("Error disabling looping: {why}");

                    respond_to_error_button(command, &ctx.http, format!("Error looping song!"))
                        .await;
                }
            }
        } else {
            match current_song.unwrap().enable_loop() {
                Ok(_) => {
                    respond_to_button(command, &ctx.http, format!("Enabled **looping!** Use **/loop** again to disable or **/skip** to skip"), true).await;
                }
                // Error enabling loop, return early
                Err(why) => {
                    println!("Error looping song: {why}");

                    respond_to_error_button(command, &ctx.http, format!("Error looping song!"))
                        .await;
                }
            }
        }
    } else {
        respond_to_error_button(
            command,
            &ctx.http,
            format!("Error looping song! Ensure Poor Jimmy is in a voice channel with **/join**"),
        )
        .await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("loop")
        .description("Enable/disable looping for the current song")
}
