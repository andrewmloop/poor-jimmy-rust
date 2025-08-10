use serenity::{
    builder::{CreateActionRow, CreateButton, CreateComponents},
    model::prelude::component::ButtonStyle,
};

pub fn create_music_buttons() -> CreateComponents {
    let clear_button = CreateButton::default()
        .custom_id("clear")
        .label("📋 Clear")
        .style(ButtonStyle::Danger)
        .to_owned();
    let resume_button = CreateButton::default()
        .custom_id("resume")
        .label("▶️ Resume")
        .style(ButtonStyle::Success)
        .to_owned();
    let pause_button = CreateButton::default()
        .custom_id("pause")
        .label("⏸️ Pause")
        .style(ButtonStyle::Primary)
        .to_owned();
    let skip_button = CreateButton::default()
        .custom_id("skip")
        .label("⏭️ Skip")
        .style(ButtonStyle::Primary)
        .to_owned();
    let loop_button = CreateButton::default()
        .custom_id("loop")
        .label("🔄 Loop")
        .style(ButtonStyle::Primary)
        .to_owned();

    let mut row = CreateActionRow::default();
    row.add_button(clear_button);
    row.add_button(resume_button);
    row.add_button(pause_button);
    row.add_button(skip_button);
    row.add_button(loop_button);

    let mut component = CreateComponents::default();
    component.add_action_row(row);

    component
}
