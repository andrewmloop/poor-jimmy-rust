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
                .description("Please provide a title to search!")
                .color(Color::DARK_RED);

            respond_to_followup(command, &ctx.http, response_embed, false).await;

            return;
        }
    };

    let title = match resolved_value {
        CommandDataOptionValue::String(value) => value.clone(),
        _ => {
            response_embed
                .description("Please provide a valid title!")
                .color(Color::DARK_RED);

            respond_to_followup(command, &ctx.http, response_embed, false).await;

            return;
        }
    };

    play_title(&ctx, &command, title).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play-title")
        .description("Play the audio from a Youtube video searching by title")
        .create_option(|option| {
            option
                .name("title")
                .description("A Youtube video title")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

async fn play_title(ctx: &Context, command: &ApplicationCommandInteraction, title: String) {
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
        let source_result = Restartable::ytdl_search(title, true).await;

        let source = match source_result {
            Ok(source) => source,
            Err(why) => {
                println!("Error grabbing Youtube single video source: {why}");

                response_embed
                    .description("Error playing song!")
                    .color(Color::DARK_RED);

                respond_to_followup(command, &ctx.http, response_embed, false).await;

                return;
            }
        };

        // Play/enqueue song
        let track = handler.enqueue_source(source.into());
        let track_title = match &track.metadata().title {
            Some(title) => title.clone(),
            None => String::from("Song"),
        };
        let track_thumbnail = &track.metadata().thumbnail;

        let response_description = format_description(track_title, should_enqueue);

        response_embed
            .description(response_description)
            .color(Color::DARK_GREEN);

        if !should_enqueue {
            if let Some(url) = track_thumbnail {
                response_embed.image(url);
            }
        }

        respond_to_followup(command, &ctx.http, response_embed, true).await;
    } else {
        response_embed
            .description(
                "Error playing song! Ensure Poor Jimmy is in a voice channel with **/join**",
            )
            .color(Color::DARK_RED);

        respond_to_followup(command, &ctx.http, response_embed, false).await;
    }
}

fn format_description(source_title: String, should_enqueue: bool) -> String {
    if should_enqueue {
        return format!("**Queued** {}!", source_title);
    } else {
        return format!("**Playing** {}!", source_title);
    }
}
