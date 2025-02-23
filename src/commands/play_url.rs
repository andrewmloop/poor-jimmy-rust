use regex::Regex;
use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::{
        application::interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOptionValue,
        },
        prelude::command::CommandOptionType,
    },
    utils::Color,
};

use songbird::input::Restartable;
use tokio::process::Command;

use crate::utils::response::respond_to_followup;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    command.defer(&ctx.http).await.expect(
        "Deferring a command response shouldn't fail. Possible change in API requirements/response",
    );

    let mut response_embed = CreateEmbed::default();

    let command_value = command.data.options.first();

    let resolved_value = match command_value {
        Some(data) => data
            .resolved
            .as_ref()
            .expect("Couldn't unwrap resolved slash command data"),
        _ => {
            response_embed
                .description("Please provide a URL to play!")
                .color(Color::DARK_RED);

            respond_to_followup(command, &ctx.http, response_embed).await;

            return;
        }
    };

    let url = match resolved_value {
        CommandDataOptionValue::String(value) => value.clone(),
        _ => {
            response_embed
                .description("Please provide a valid URL!")
                .color(Color::DARK_RED);

            respond_to_followup(command, &ctx.http, response_embed).await;

            return;
        }
    };

    play_url(&ctx, &command, url).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play-url")
        .description("Play the audio from a Youtube video or playlist URL")
        .create_option(|option| {
            option
                .name("url")
                .description("A Youtube video/playlist URL")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

async fn play_url(ctx: &Context, command: &ApplicationCommandInteraction, url: String) {
    let mut response_embed = CreateEmbed::default();

    // Validate its a valid Youtube URL
    if !is_valid_youtube_url(&url) {
        response_embed
            .description("Please provide a valid Youtube URL. Valid URLs include **/watch** or **/playlist**")
            .color(Color::DARK_RED);

        respond_to_followup(command, &ctx.http, response_embed).await;

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

                            respond_to_followup(command, &ctx.http, response_embed).await;

                            return;
                        }
                    }
                }
                Err(why) => {
                    println!("Error getting playlist details: {why}");

                    response_embed
                        .description("Error queueing playlist")
                        .color(Color::DARK_RED);

                    respond_to_followup(command, &ctx.http, response_embed).await;

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

            respond_to_followup(command, &ctx.http, response_embed).await;

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

                    respond_to_followup(command, &ctx.http, response_embed).await;

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

    respond_to_followup(command, &ctx.http, response_embed).await;
}

fn format_description(source_title: String, should_enqueue: bool) -> String {
    if should_enqueue {
        return format!("**Queued** {}!", source_title);
    } else {
        return format!("**Playing** {}!", source_title);
    }
}

// fn contains_youtube_domain(url: &String) -> bool {
//     url.contains("youtube.com") || url.contains("youtu.be")
// }

// fn contains_watch_endpoint(url: &String) -> bool {
//     url.contains("/watch") || url.contains("/playlist")
// }

fn is_valid_youtube_url(url: &String) -> bool {
    url.contains("youtu.be")
        || (url.contains("youtube.com") && (url.contains("/watch") || url.contains("/playlist")))
}

#[cfg(test)]
mod tests {
    use crate::commands::play_url::is_valid_youtube_url;

    use super::format_description;

    #[test]
    fn it_formats_description_queued() {
        let title = String::from("Heat Waves");

        let formatted = format_description(title.clone(), true);
        assert_eq!(format!("**Queued** {}!", title), formatted);
    }

    #[test]
    fn it_formats_description_playing() {
        let title = String::from("Heat Waves");

        let formatted = format_description(title.clone(), false);
        assert_eq!(format!("**Playing** {}!", title), formatted);
    }

    #[test]
    fn it_validates_valid_youtube_urls() {
        let valid_watch_url = String::from("https://www.youtube.com/watch?id=12345");
        let valid_share_url = String::from("https://youtu.be/e7qtC_e8Jxc?si=mtCnq8iVc253P89M");
        let valid_playlist_url = String::from("https://www.youtube.com/playlist?id=12345");

        assert_eq!(true, is_valid_youtube_url(&valid_watch_url));
        assert_eq!(true, is_valid_youtube_url(&valid_share_url));
        assert_eq!(true, is_valid_youtube_url(&valid_playlist_url));
    }

    #[test]
    fn it_validates_invalid_youtube_urls() {
        let invalid_url = String::from("https://www.you.tube.com/watch?id=12345");
        let another_invalid_url =
            String::from("https://www.youtube.com/results?search_query=title");

        assert_eq!(false, is_valid_youtube_url(&invalid_url));
        assert_eq!(false, is_valid_youtube_url(&another_invalid_url));
    }

    // #[test]
    // fn it_contains_youtube_domain() {

    //     assert_eq!(true, contains_youtube_domain(&valid_url));
    //     assert_eq!(true, contains_youtube_domain(&another_valid_url));
    //     assert_eq!(false, contains_youtube_domain(&invalid_url));
    // }

    // #[test]
    // fn it_contains_watch_endpoint() {
    //     let watch_endpoint = String::from("https://www.youtube.com/watch?id=12345");

    //     assert_eq!(true, contains_watch_endpoint(&watch_endpoint));
    //     assert_eq!(true, contains_watch_endpoint(&playlist_endpoint));
    //     assert_eq!(false, contains_watch_endpoint(&invalid_endpoint));
    // }
}
