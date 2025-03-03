use bevy::app::App;
use bevy::prelude::Plugin;
use crate::ui::interaction::interaction_style::InteractionStylePlugin;

pub mod interaction_style;
pub mod interaction_events;


pub struct CustomInteractionPlugin;

impl Plugin for CustomInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_plugins(
                (
                    InteractionStylePlugin,
                )
            );
    }
}