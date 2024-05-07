use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    utils::Color,
};

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

        // Grab the queue and make sure its not empty
        let current_queue = handler.queue().current_queue();
        if current_queue.is_empty() {
            response_embed
                .description("The queue is **empty!**")
                .color(Color::DARK_GREEN);

            respond_to_command(command, &ctx.http, response_embed).await;

            return;
        }

        // Transform the Vec of TrackHandles into a Vec of titles
        let queue_titles: Vec<String> = current_queue
            .iter()
            .map(|track| {
                track
                    .metadata()
                    .title
                    .clone()
                    .unwrap_or_else(|| "Mystery song".to_string())
            })
            .collect();

        // Build the response description string.
        let response_description = format_queue_description(queue_titles);

        response_embed
            .description(response_description)
            .color(Color::DARK_GREEN);
    } else {
        response_embed
            .description(
                "Error listing queue! Ensure Poor Jimmy is in a voice channel with **/join**",
            )
            .color(Color::DARK_RED);
    }

    respond_to_command(command, &ctx.http, response_embed).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("Display the current queue of songs")
}

fn format_queue_description(list_of_titles: Vec<String>) -> String {
    let mut description = String::new();

    for (index, title) in list_of_titles.iter().enumerate() {
        description.push_str(format!("**{}:** {}\n", index + 1, title).as_str())
    }

    description
}
