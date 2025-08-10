use serenity::{
    builder::CreateEmbed,
    http::Http,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::message_component::MessageComponentInteraction,
    },
    utils::Color,
};

use crate::components::music_buttons::create_music_buttons;

/// Respond to an ApplicationCommandInteraction with the given CreateEmbed.
///
/// This assumes the command has not been deferred or responded to yet. If the
/// command may have been deferred use `respond_to_follow` instead.
pub async fn respond_to_command(
    command: &ApplicationCommandInteraction,
    http: &Http,
    content: String,
    include_buttons: bool,
) {
    command
        .create_interaction_response(http, |response| {
            response.interaction_response_data(|data| {
                data.set_embed(CreateEmbed::default()
                    .color(Color::DARK_GREEN)
                    .description(content)
                    .to_owned());

                if include_buttons {
                    data.set_components(create_music_buttons());
                }

                data
            })
        })
        .await
        .expect("Sending a command response followup shouldn't fail. Possible change in API requirements/response");
}

pub async fn respond_to_error(
    command: &ApplicationCommandInteraction,
    http: &Http,
    content: String,
) {
    command
        .create_interaction_response(http, |response| {
            response.interaction_response_data(|data| {
                data.set_embed(CreateEmbed::default()
                    .color(Color::DARK_RED)
                    .description(content)
                    .to_owned());

                data
            })
        })
        .await
        .expect("Sending a command response followup shouldn't fail. Possible change in API requirements/response");
}

pub async fn respond_to_button(
    command: &MessageComponentInteraction,
    http: &Http,
    content: String,
    include_buttons: bool,
) {
    command
        .create_interaction_response(http, |response| {
            response.interaction_response_data(|data| {
                data.set_embed(CreateEmbed::default()
                    .color(Color::DARK_GREEN)
                    .description(content)
                    .to_owned());

                if include_buttons {
                    data.set_components(create_music_buttons());
                }

                data
            })
        })
        .await
        .expect("Sending a command response followup shouldn't fail. Possible change in API requirements/response");
}

pub async fn respond_to_error_button(
    command: &MessageComponentInteraction,
    http: &Http,
    content: String,
) {
    command
        .create_interaction_response(http, |response| {
            response.interaction_response_data(|data| {
                data.set_embed(CreateEmbed::default()
                    .color(Color::DARK_RED)
                    .description(content)
                    .to_owned());

                data
            })
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
    include_buttons: bool,
) {
    command
        .create_followup_message(http, |response| {
            response.set_embed(content);

            if include_buttons {
                response.set_components(create_music_buttons());
            }

            response
        })
        .await
        .expect("Sending a command response followup shouldn't fail. Possible change in API requirements/response");
}
