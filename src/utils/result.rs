use serenity::utils::Color;

/// The result of executing a command. Holds the content the bot responds with as well as whether the result of the command was a success. This utility class allows for calculating what the interaction response will look like.
///
/// E.g. calculating the color of the response embed depending on whether the result was a sucesss.
#[derive(Clone, Default)]
pub struct CommandResponse {
    description: String,
    color: Color,
}

impl CommandResponse {
    pub fn new() -> CommandResponse {
        CommandResponse::default()
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_description(&self) -> String {
        self.description.to_owned()
    }
}
