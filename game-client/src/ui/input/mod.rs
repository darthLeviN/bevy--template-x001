use bevy::prelude::*;
pub mod focus;
pub mod input_map;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(focus::UiFocusPlugin);
    }
}