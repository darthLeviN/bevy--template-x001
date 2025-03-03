use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::input::keyboard::Key;
use bevy::prelude::*;
use smol_str::SmolStr;
use crate::event_system::{HandledEventExt, UnhandledEventTriggerExt};
use crate::ui::input::focus::InputFocus;
use crate::ui::input::input_map::MappedInputEvent;

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app : &mut App) {
        // Types
        app.register_type::<BasicTextInput>();

        // Systems
        app.add_systems(PostUpdate, text_update_system);

        // Observers
        app.add_observer(basic_text_input_observer);


    }
}


#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum TextInputChangeEvent {
    Append(String),
    Replace(String),
}

impl TextInputChangeEvent {
    fn append_str(&mut self, value: &str) {
        match self {
            TextInputChangeEvent::Append(current) => {
                current.push_str(value);
            }

            _ => {
                *self = TextInputChangeEvent::Append(String::from(value))
            }
        }
    }
}



#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct BasicTextInput {
    pub placeholder_text: String,
    pub next_text: Option<TextInputChangeEvent>,
}

fn basic_text_input_observer(mut trigger: Trigger<MappedInputEvent>, mut query: Query<&mut BasicTextInput>, input_focus: Res<InputFocus>) {
    if let Some(Key::Character(cc)) = trigger.event().logical_key.clone() {
        let entity = trigger.entity();
        let current_focus = input_focus.0;
        if Some(entity) == current_focus {
            if let Ok(mut text_input) = query.get_mut(entity) {
                if let Some(next_text) = text_input.next_text.as_mut() {
                    next_text.append_str(&cc.to_string());
                } else {
                    text_input.next_text = Some(TextInputChangeEvent::Append(String::from(cc)));
                }
            }
        }
    }
}

fn text_update_system(mut query: Query<(&mut Text, &mut BasicTextInput), Changed<BasicTextInput>>) {
    for (mut base_text, mut input_text) in query.iter_mut() {
        if let Some(next_text) = input_text.next_text.take() {
            match next_text {
                TextInputChangeEvent::Append(text) => {
                    base_text.push_str(&text);
                }
                TextInputChangeEvent::Replace(text) => {
                    base_text.clear();
                    base_text.push_str(&text);
                }
            }
        }
    }
}