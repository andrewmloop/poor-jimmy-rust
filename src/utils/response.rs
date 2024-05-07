use serenity::{
    builder::CreateEmbed, http::Http,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

/// Respond to an ApplicationCommandInteraction with the given CreateEmbed.
///
/// This assumes the command has not been deferred or responded to yet. If the
/// command may have been deferred use `respond_to_follow` instead.
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
        .expect("Sending a command response followup shouldn't fail. Possible change in API requirements/response");
}

/// Respond to a deferred ApplicationCommandInteraction with the given
/// CreateEmbed.
///
/// This assumes the command has been deferred. If the command is not deferred
/// use `respond_to_command` instead.
pub async fn respond_to_followup(
    command: &ApplicationCommandInteraction,
    http: &Http,
    content: CreateEmbed,
) {
    command
        .create_followup_message(http, |response| response.set_embed(content))
        .await
        .expect("Sending a command response followup shouldn't fail. Possible change in API requirements/response");
}
