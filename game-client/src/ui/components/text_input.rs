use std::borrow::Cow::{Borrowed, Owned};
use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::input::keyboard::Key;
use bevy::prelude::*;
use smol_str::SmolStr;
use crate::event_system::{HandledEventExt, UnhandledEventTriggerExt};
use crate::ui::components::text_creator::TextCreator;
use crate::ui::input::focus::InputFocus;
use crate::ui::input::input_map::MappedInputEvent;

pub struct TextInputPlugin;

const CLEAR_TEXT_KEY: Key = Key::Soft1;

impl Plugin for TextInputPlugin {
    fn build(&self, app : &mut App) {
        // Types
        app.register_type::<BasicTextInput>();

        // Systems
        // app.add_systems(PostUpdate, text_update_system);

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
}

fn basic_text_input_observer(mut trigger: Trigger<MappedInputEvent>, mut commands: Commands) {
    let entity = trigger.entity();
    if let Some(keyboard_input) = trigger.event().keyboard_input.as_ref() {
        if !keyboard_input.repeat && !keyboard_input.state.is_pressed() {
            return;
        }

        match keyboard_input.logical_key.clone() {
            Key::Character(cc) => {

                commands.queue(move |mut world: &mut World| {
                    let mut input_focus = world.resource::<InputFocus>();
                    let current_input_focus = input_focus.0;
                    if Some(entity) == current_input_focus {
                        if let Some(mut text_creator) = world.get_mut::<TextCreator>(entity).as_mut() {
                            text_creator.text.to_mut().push_str(cc.as_str());
                        }
                    }
                });
            }
            CLEAR_TEXT_KEY => {
                commands.queue(move |mut world: &mut World| {
                    let mut input_focus = world.resource::<InputFocus>();
                    let current_input_focus = input_focus.0;
                    if Some(entity) == current_input_focus {
                        if let Some(mut text_creator) = world.get_mut::<TextCreator>(entity).as_mut() {
                            text_creator.text = Borrowed("");
                        }
                    }
                });
            }

            Key::Space => {
                commands.queue(move |mut world: &mut World| {
                    let mut input_focus = world.resource::<InputFocus>();
                    let current_input_focus = input_focus.0;
                    if Some(entity) == current_input_focus {
                        if let Some(mut text_creator) = world.get_mut::<TextCreator>(entity).as_mut() {
                            text_creator.text.to_mut().push(' ');
                        }
                    }
                });
            }

            Key::Backspace => {
                commands.queue(move |mut world: &mut World| {
                    let mut input_focus = world.resource::<InputFocus>();
                    let current_input_focus = input_focus.0;
                    if Some(entity) == current_input_focus {
                        if let Some(mut text_creator) = world.get_mut::<TextCreator>(entity).as_mut() {
                            text_creator.text.to_mut().pop();
                        }
                    }
                });
            }

            _ => {}
        }
    }

}