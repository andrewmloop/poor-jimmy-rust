use regex::Regex;
use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
    utils::Color,
};

use songbird::input::{self};
use tokio::process::Command;

use crate::utils::message::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut response_embed = CreateEmbed::default();

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
            response_embed
                .description("Please provide a valid Youtube URL")
                .color(Color::DARK_RED);

            respond_to_command(command, &ctx.http, response_embed).await;

            return;
        }
    };

    // Validate its a valid Youtube URL
    if !string_option.contains("youtube.com") {
        response_embed
            .description("Please provide a valid Youtube URL")
            .color(Color::DARK_RED);

        respond_to_command(command, &ctx.http, response_embed).await;

        return;
    }

    // Grab the voice client registered with Serentiy's shard key-value store
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    // Grab the active Call for the command's guild
    if let Some(call) = manager.get(guild_id) {
        let mut handler = call.lock().await;

        // If url is a Youtube playlist
        if string_option.contains("/playlist") {
            // Use yt-dlp to produce json of all the individual videos
            let playlist_result = Command::new("yt-dlp")
                .args(["-j", "--flat-playlist", &string_option])
                .output()
                .await;

            // Grab only the urls
            let playlist = match playlist_result {
                Ok(playlist) => {
                    // println!("Playlist songs {:?}", playlist);
                    String::from_utf8(playlist.stdout).expect("Error parsing playlist json")
                }
                Err(why) => {
                    println!("Error getting playlist details: {why}");

                    response_embed
                        .description("Error queueing playlist")
                        .color(Color::DARK_RED);

                    respond_to_command(command, &ctx.http, response_embed).await;

                    return;
                }
            };

            println!("{:?}", playlist);

            // Split the string of urls into a vec
            let re = Regex::new(r#""url": "(https://www.youtube.com/watch\?v=[A-Za-z0-9]{11})""#)
                .expect("Error building Youtube URL regex");

            let urls: Vec<String> = re
                .captures_iter(&playlist)
                .map(|capture| capture[1].to_string())
                .collect();

            println!("{:?}", urls);

            // Queueing all the urls may take some time, notify the channel
            response_embed
                .description("Queueing playlist. This may take some time...")
                .color(Color::DARK_GREEN);

            respond_to_command(command, &ctx.http, response_embed).await;

            // Attempt to enqueue as many of the videos we can
            let mut num_queued_songs = 0;

            for url in urls {
                let source = match input::ytdl(url).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Error grabbing playlist source: {why}");
                        continue;
                    }
                };

                handler.enqueue_source(source);
                num_queued_songs += 1;
            }

            // Send response with number of queued songs
            let _ = command
                .channel_id
                .send_message(&ctx.http, |message| {
                    message.add_embed(|embed| {
                        embed
                            .description(format!(
                                "{} songs **queued!** Use **/list** to view them",
                                num_queued_songs
                            ))
                            .color(Color::DARK_GREEN)
                    })
                })
                .await;

            return;
        } else if string_option.contains("/watch") {
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

            response_embed
                .description(response_description)
                .color(Color::DARK_GREEN);

            respond_to_command(command, &ctx.http, response_embed).await;
        } else {
            response_embed
                .description("Error playing song")
                .color(Color::DARK_RED);

            respond_to_command(command, &ctx.http, response_embed).await;
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play the audio from a Youtube video")
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
