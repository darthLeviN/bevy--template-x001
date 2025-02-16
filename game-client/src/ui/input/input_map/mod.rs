use std::collections::{HashMap, HashSet};
use std::ptr::hash;
use bevy::ecs::system::RunSystemOnce;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use maplit::hashset;
use crate::event_system::HandledEventExt;

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputMaps::default());
        app.insert_resource(KeyStates::default());
        app.add_systems(PostUpdate, managed_keyboard_input_system);
        app.add_event::<ManagedKeyboardInput>();
        app.add_event::<MappedInputEvent>();
    }
}

#[derive(Resource)]
pub struct InputMaps {
    map: HashMap<String, InputContext>,
    pub(crate) comparison_cache: HashMap<InputValue, HashSet<String>>
}

#[derive(Clone, Debug, Default, Component)]
pub struct InputContext {
    pub values: HashSet<InputValue>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum InputValue {
    Keyboard(KeyboardInput),
}

impl Into<ManagedKeyboardInput> for KeyboardInput {
    fn into(self) -> ManagedKeyboardInput {
        ManagedKeyboardInput(self)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Event)]
pub struct ManagedKeyboardInput(pub(crate) KeyboardInput);

impl InputValue {
    pub fn matches_keycode(&self, other: &Self) -> bool {
        match (self, other) {
            (InputValue::Keyboard(a), InputValue::Keyboard(b)) => a.key_code == b.key_code,
        }
    }
}

impl InputMaps {

    fn add_input_cache(&mut self, key: String, input_value: InputValue) {
        if let Some(cache) = self.comparison_cache.get_mut(&input_value) {
            cache.insert(key);
        } else {
            let mut hash_set = HashSet::new();
            hash_set.insert(key);
            self.comparison_cache.insert(input_value, hash_set);
        }
    }
    fn add_input_value(&mut self, key: String, input_value: InputValue) {
        if let Some(input_context) = self.map.get_mut(&key) {
            if !input_context.values.contains(&input_value) {
                input_context.values.insert(input_value.clone());
                self.add_input_cache(key, input_value);
            }
        } else {
            let mut hash_set = HashSet::new();
            hash_set.insert(input_value.clone());
            self.map.insert(
                key.clone(),
                InputContext {
                values: hash_set,
            });

            self.add_input_cache(key, input_value);
        }
    }

    fn recompute_cache(&mut self) {
        self.comparison_cache.clear();
        for (key, input_context) in self.map.iter() {
            for input_value in input_context.values.iter() {
                if let Some(hash_set) = self.comparison_cache.get_mut(input_value) {
                    hash_set.insert(key.clone());
                } else {
                    self.comparison_cache.insert(input_value.clone(), hashset!{key.clone()});
                }
            }
        }
    }
}

impl Default for InputMaps {
    fn default() -> Self {
        let mut map: HashMap<String, InputContext> = HashMap::new();

        map.insert(
            "ui_cancel".into(),
            InputContext {
                values: hashset!{InputValue::Keyboard(
                    KeyboardInput {
                        key_code: KeyCode::Escape,
                        logical_key: Key::Escape,
                        state: ButtonState::Pressed,
                        repeat: false,
                        window: Entity::PLACEHOLDER
                    }
                )}
            }
        );

        map.insert(
            "ui_confirm".into(),
            InputContext {
                values: hashset!{InputValue::Keyboard(KeyboardInput {
                    key_code: KeyCode::Enter,
                    logical_key: Key::Enter,
                    state: ButtonState::Pressed,
                    repeat: false,
                    window: Entity::PLACEHOLDER
                })}
            },
        );

        map.insert(
            "ui_focus_release".into(),
            InputContext {
                values: hashset!{InputValue::Keyboard(
                    KeyboardInput {
                        key_code: KeyCode::Escape,
                        logical_key: Key::Escape,
                        state: ButtonState::Pressed,
                        repeat: false,
                        window: Entity::PLACEHOLDER
                    }
                )}
            }
        );

        let mut ret = Self {
            map,
            comparison_cache: HashMap::new(),
        };

        ret.recompute_cache();

        return ret;
    }
}

#[derive(Clone, Debug, Component)]
pub struct MappedInputEvent {
    pub keys: HashSet<String>,
    pub(crate) is_handled: bool,
}

impl Event for MappedInputEvent {
    type Traversal = &'static Parent;
    const AUTO_PROPAGATE: bool = true;
}

impl HandledEventExt for MappedInputEvent {
    fn is_handled(&self) -> bool {
        self.is_handled
    }

    fn set_handled(&mut self, handled: bool) {
        self.is_handled = handled;
    }
}

#[derive(Resource, Default)]
pub struct KeyStates {
    pub states: HashMap<KeyCode, ButtonState>,
}

impl KeyStates {
    pub fn set_state(&mut self, key_code: KeyCode, state: ButtonState) -> Option<ButtonState> {
        let prev = self.states.insert(key_code, state).unwrap_or(ButtonState::Released);
        return if prev != state {
            Some(prev)
        } else {
            None
        }
    }

    pub fn get_state(&self, key_code: KeyCode) -> ButtonState {
        self.states.get(&key_code).unwrap_or(&ButtonState::Released).clone()
    }
}

pub fn managed_keyboard_input_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut key_states: ResMut<KeyStates>,
    mut managed_event_writer: EventWriter<ManagedKeyboardInput>,
    mut mapped_event_writer: EventWriter<MappedInputEvent>,
    input_maps: Res<InputMaps>) {

    for event in keyboard_input_events.read() {
        if let Some(new_state) = key_states.set_state(event.key_code, event.state) {
            info!("Key {:?} changed state from {:?} to {:?}", event.key_code, new_state, event.state);
            let mut event = event.clone();
            event.window = Entity::PLACEHOLDER;
            if let Some(input_context) = input_maps.comparison_cache.get(&InputValue::Keyboard(event.clone())) {
                let mapped_input = MappedInputEvent {
                    keys: input_context.clone(),
                    is_handled: false
                };
                mapped_event_writer.send(mapped_input);
            }
            let managed_input = ManagedKeyboardInput(event);
            managed_event_writer.send(managed_input);

        }
    }
}