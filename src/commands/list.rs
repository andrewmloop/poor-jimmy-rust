use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

use crate::utils::response::{respond_to_command, respond_to_error};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.");

    let guild_id = command.guild_id.unwrap();

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        // Grab the queue and make sure its not empty
        let current_queue = handler.queue().current_queue();
        if current_queue.is_empty() {
            respond_to_command(
                command,
                &ctx.http,
                format!("The queue is **empty!**"),
                false,
            )
            .await;

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

        respond_to_command(command, &ctx.http, response_description, true).await;
    } else {
        respond_to_error(
            command,
            &ctx.http,
            format!("Error listing queue! Ensure Poor Jimmy is in a voice channel with **/join**"),
        )
        .await;
    }
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
