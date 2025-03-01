use bevy::prelude::*;

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, _: &mut App) {

    }
}


#[derive(Component, Clone, Reflect, Default)]
pub struct TextInput;

