use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
    utils::Color,
};

use songbird::input::{self};

use crate::utils::result::CommandResponse;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> CommandResponse {
    let response: CommandResponse;

    // Grab the value passed into the slash command
    let command_option = command
        .data
        .options
        .first()
        .expect("Expected play command string option")
        .resolved
        .as_ref()
        .expect("Expected play command string object");

    // Validate the value is a string
    let string_option = match command_option {
        CommandDataOptionValue::String(option) => option,
        _ => {
            response = CommandResponse::new()
                .description(String::from("Please provide a valid Youtube URL"))
                .color(Color::DARK_RED)
                .clone();

            return response;
        }
    };

    // Validate its a valid Youtube URL
    if !string_option.contains("youtube.com/watch") {
        response = CommandResponse::new()
            .description(String::from("Please provide a valid Youtube URL"))
            .color(Color::DARK_RED)
            .clone();

        return response;
    }

    // Grab the voice client registered with Serentiy's shard key-value store
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();
    println!("Play command guild id: {guild_id}");

    // Grab the active Call for the command's guild
    if let Some(call) = manager.get(guild_id) {
        let mut handler = call.lock().await;

        // If a song is currently playing, we'll add the new song to the queue
        let should_enqueue = match handler.queue().current() {
            Some(_) => true,
            None => false,
        };

        // Get the audio source for the URL
        let source = input::ytdl(string_option)
            .await
            .expect("Failure grabbing Youtube URL source");

        let source_metadata = source.metadata.clone();
        let source_title = match source_metadata.title {
            Some(title) => title,
            None => String::from("Song"),
        };

        // Play/enqueue song
        handler.enqueue_source(source);

        let response_description = format_description(source_title, should_enqueue);

        response = CommandResponse::new()
            .description(response_description)
            .color(Color::DARK_GREEN)
            .clone();
    } else {
        response = CommandResponse::new()
            .description(String::from("Error playing song"))
            .color(Color::DARK_RED)
            .clone();
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Plays the audio from a Youtube video")
        .create_option(|option| {
            option
                .name("url")
                .description("A Youtube URL")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

fn format_description(source_title: String, should_enqueue: bool) -> String {
    if should_enqueue {
        return format!("**Queued** {}!", source_title);
    } else {
        return format!("**Playing** {}!", source_title);
    }
}
