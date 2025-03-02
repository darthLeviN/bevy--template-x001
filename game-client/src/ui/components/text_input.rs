use bevy::prelude::*;

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app : &mut App) {
        // app.register_type::<BasicTextInput>();
    }
}


// #[derive(Component, Clone, Reflect, Default)]
// #[reflect(Component)]
// pub struct BasicTextInput;
//
//
// pub fn test_input_observer(mut trigger: Trigger<>)