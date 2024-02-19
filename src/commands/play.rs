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

use songbird::input::Restartable;
use tokio::process::Command;

use crate::utils::message::respond_to_command;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut response_embed = CreateEmbed::default();

    // Grab the command option
    let command_option = command.data.options.first();

    // Grab option name
    let option_name = match command_option {
        Some(option) => option.name.clone(),
        None => {
            response_embed
                .description("Provide a valid value to the **url** or **search** option!")
                .color(Color::DARK_RED);

            respond_to_command(command, &ctx.http, response_embed).await;

            return;
        }
    };

    // Grab the option value
    let option_value = match command_option {
        Some(option) => match option.resolved.as_ref() {
            Some(data) => data,
            None => {
                response_embed
                    .description("Provide a valid value to the **url** or **search** option!")
                    .color(Color::DARK_RED);

                respond_to_command(command, &ctx.http, response_embed).await;

                return;
            }
        },
        None => {
            response_embed
                .description("Provide a valid value to the **url** or **search** option!")
                .color(Color::DARK_RED);

            respond_to_command(command, &ctx.http, response_embed).await;

            return;
        }
    };

    // Validate the value is a string
    let string_option = match option_value {
        CommandDataOptionValue::String(option) => option.clone(),
        _ => {
            response_embed
                .description("Please provide a valid Youtube URL or search!")
                .color(Color::DARK_RED);

            respond_to_command(command, &ctx.http, response_embed).await;

            return;
        }
    };

    // Branch command depending on command option
    match option_name.as_str() {
        "url" => play_url(&ctx, &command, string_option).await,
        "search" => play_search(&ctx, &command, string_option).await,
        _ => {
            // This should never happen, log this to stderr
            eprintln!("[ERROR] Unknown option given to play command {option_name}")
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play the audio from a Youtube video or playlist")
        .create_option(|option| {
            option
                .name("url")
                .description("A Youtube URL")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("search")
                .description("Search for a Youtube video by matching its title")
                .kind(CommandOptionType::String)
        })
}

async fn play_url(ctx: &Context, command: &ApplicationCommandInteraction, url: String) {
    let mut response_embed = CreateEmbed::default();

    // Validate its a valid Youtube URL
    if !url.contains("youtube.com") && !url.contains("youtu.be") {
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
        if url.contains("/playlist") {
            // Use yt-dlp to produce json of all the individual videos
            let playlist_result = Command::new("yt-dlp")
                .args(["-j", "--flat-playlist", &url])
                .output()
                .await;

            // Grab only the urls
            let playlist = match playlist_result {
                Ok(playlist) => {
                    let playlist_json_result = String::from_utf8(playlist.stdout);

                    match playlist_json_result {
                        Ok(playlist) => playlist,
                        Err(why) => {
                            println!("Error parsing playlist json: {why}");

                            response_embed
                                .description("Error queueing playlist")
                                .color(Color::DARK_RED);

                            respond_to_command(command, &ctx.http, response_embed).await;

                            return;
                        }
                    }
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

            // Split the string of urls into a vec
            let re = Regex::new(r#""url": "(https://www.youtube.com/watch\?v=[A-Za-z0-9]{11})""#)
                .expect("Error building Youtube URL regex");

            let urls: Vec<String> = re
                .captures_iter(&playlist)
                .map(|capture| capture[1].to_string())
                .collect();

            // Queueing all the urls may take some time, notify the channel
            response_embed
                .description("**Queueing** playlist. This may take some time. Some commands will be unavilable until completed.")
                .color(Color::DARK_GREEN);

            respond_to_command(command, &ctx.http, response_embed).await;

            // Attempt to enqueue as many of the videos we can
            let mut num_queued_songs = 0;

            for url in urls {
                let source = match Restartable::ytdl(url, true).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Error grabbing playlist source. Most likely due to an unavailable/hidden video: {why}");
                        continue;
                    }
                };

                handler.enqueue_source(source.into());
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
        } else {
            // If a song is currently playing, we'll add the new song to the queue
            let should_enqueue = match handler.queue().current() {
                Some(_) => true,
                None => false,
            };

            // Get the audio source for the URL
            let source_result = Restartable::ytdl(url, true).await;

            let source = match source_result {
                Ok(source) => source,
                Err(why) => {
                    println!("Error grabbing Youtube single video source: {why}");

                    response_embed
                        .description("Error playing song")
                        .color(Color::DARK_RED);

                    respond_to_command(command, &ctx.http, response_embed).await;

                    return;
                }
            };

            // Play/enqueue song
            let track = handler.enqueue_source(source.into());
            let track_title = match &track.metadata().title {
                Some(title) => title.clone(),
                None => String::from("Song"),
            };

            let response_description = format_description(track_title, should_enqueue);

            response_embed
                .description(response_description)
                .color(Color::DARK_GREEN);
        }
    } else {
        response_embed
            .description(
                "Error playing song! Ensure Poor Jimmy is in a voice channel with **/join**",
            )
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

async fn play_search(ctx: &Context, command: &ApplicationCommandInteraction, search: String) {
    let mut response_embed = CreateEmbed::default();

    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    // Grab the active Call for the command's guild
    if let Some(call) = manager.get(guild_id) {
        let mut handler = call.lock().await;

        let should_enqueue = match handler.queue().current() {
            Some(_) => true,
            None => false,
        };

        // Get the audio source for the URL
        let source_result = Restartable::ytdl_search(search, true).await;

        let source = match source_result {
            Ok(source) => source,
            Err(why) => {
                println!("Error grabbing Youtube single video source: {why}");

                response_embed
                    .description("Error playing song")
                    .color(Color::DARK_RED);

                respond_to_command(command, &ctx.http, response_embed).await;

                return;
            }
        };

        // Play/enqueue song
        let track = handler.enqueue_source(source.into());
        let track_title = match &track.metadata().title {
            Some(title) => title.clone(),
            None => String::from("Song"),
        };

        let response_description = format_description(track_title, should_enqueue);

        response_embed
            .description(response_description)
            .color(Color::DARK_GREEN);
    } else {
        response_embed
            .description(
                "Error playing song! Ensure Poor Jimmy is in a voice channel with **/join**",
            )
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

fn format_description(source_title: String, should_enqueue: bool) -> String {
    if should_enqueue {
        return format!("**Queued** {}!", source_title);
    } else {
        return format!("**Playing** {}!", source_title);
    }
}
