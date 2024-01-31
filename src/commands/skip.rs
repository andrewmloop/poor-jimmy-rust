use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

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

        let queue_length = handler.queue().len();

        // Attempt to skip the currently playing song
        let skip_result = match handler.queue().current() {
            Some(track) => track.stop(),
            None => {
                response = CommandResponse::new()
                    .description(String::from("There is no song currently playing!"))
                    .color(Color::DARK_RED)
                    .clone();

                return response;
            }
        };

        match skip_result {
            // The song was successfully skipped. Notify the channel if the
            // queue is now empty
            Ok(_) => {
                // We had to utilize the length of the queue before the skip
                // was attempted or else we have to do some weird sequence of
                // commands to ensure the queue is properly updated. Skipping/
                // stopping a track takes a while to remove it from the queue
                // also, manually removing tracks doesn't trigger the next
                // track to play
                if queue_length == 1 {
                    response = CommandResponse::new()
                        .description(String::from("The queue has **ended!**"))
                        .color(Color::DARK_GREEN)
                        .clone();
                } else {
                    response = CommandResponse::new()
                        .description(String::from("Song **skipped!**"))
                        .color(Color::DARK_GREEN)
                        .clone();
                }
            }
            Err(why) => {
                println!("Error skipping track: {why}");
                response = CommandResponse::new()
                    .description(String::from("Error skipping song!"))
                    .color(Color::DARK_RED)
                    .clone();
            }
        };
    } else {
        response = CommandResponse::new()
            .description(String::from("Error skipping song!"))
            .color(Color::DARK_RED)
            .clone();
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("skip")
        .description("Skips the currently playing song")
}
