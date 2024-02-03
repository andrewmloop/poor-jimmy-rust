use serenity::{
    builder::CreateEmbed, http::Http,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn respond_to_command(
    command: &ApplicationCommandInteraction,
    http: &Http,
    content: CreateEmbed,
) {
    command
        .create_interaction_response(http, |response| {
            response.interaction_response_data(|data| data.set_embed(content))
        })
        .await
        .expect("Error sending 'Queueing playlist' message");
}
