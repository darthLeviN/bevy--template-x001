use std::collections::{HashMap, HashSet};
use bevy::prelude::*;

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputMaps::default());
    }
}

#[derive(Resource)]
pub struct InputMaps {
    map: HashMap<String, InputContext>
}

#[derive(Hash, Clone, Debug, Default, Component)]
pub struct InputContext {
    pub values: Vec<InputValue>,
}

#[derive(Hash, Clone, Debug)]
pub enum InputValue {
    Keyboard(KeyCode),
}

impl InputValue {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (InputValue::Keyboard(a), InputValue::Keyboard(b)) => a == b,
        }
    }
}

impl Default for InputMaps {
    fn default() -> Self {
        let mut map: HashMap<String, InputContext> = HashMap::new();

        map.insert(
            "ui_cancel".into(),
            InputContext {
                values: vec![InputValue::Keyboard(KeyCode::Escape)]
            }
        );

        map.insert(
            "ui_confirm".into(),
            InputContext {
                values: vec![InputValue::Keyboard(KeyCode::Enter)]
            },
        );

        map.insert(
            "ui_focus_release".into(),
            InputContext {
                values: vec![InputValue::Keyboard(KeyCode::Escape)]
            }
        );

        Self {
            map
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct MappedInputEvent {
    pub keys: HashSet<String>
}

impl Event for MappedInputEvent {
    type Traversal = &'static Parent;
    const AUTO_PROPAGATE: bool = true;
}

#[derive(Clone, Event,Debug)]
pub struct UnhandledInputEvent {
    event: MappedInputEvent
}