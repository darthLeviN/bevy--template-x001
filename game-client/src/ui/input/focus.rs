use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::UiSystem::Focus;
use bevy::utils::tracing::event;
use crate::ui::input::input_map::{InputValue, MappedInputEvent};

pub struct UiFocusPlugin;

impl Plugin for UiFocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(click_focus_change_observer);
        app.register_system(focus_release_system);
        app.register_type::<InputFocusPolicy>();
        app.insert_resource(InputFocus(None));
    }
}


#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub enum InputFocusPolicy {
    None,
    All
}

#[derive(Event, Clone, Debug)]
pub enum ReleaseFocusEvent {
    ToParent{ entity: Entity, allow_none: bool},
    All,
}

#[derive(Resource, Clone, Debug)]
pub struct InputFocus(pub Option<Entity>);


// Manages focus change based on click.
pub fn click_focus_change_observer(mut trigger: Trigger<Pointer<Click>>, query: Query<Entity>, mut input_focus: ResMut<InputFocus>) {
    let trigger_entity = trigger.entity();
    if let Ok((entity)) = query.get(trigger_entity) {
        info!("Focus changed to: {:?}", entity);
        input_focus.0 = Some(entity);
    }
}
//
// pub fn default_focus_inputs(
//     mut trigger: Trigger<MappedInputEvent>,
//     input_focus: ResMut<InputFocus>,
//     mut event_writer: EventWriter<ReleaseFocusEvent>) {
//     if let Some(entity) = input_focus.0 {
//         if trigger.entity() == entity {
//             event_writer.send(ReleaseFocusEvent::ToParent{entity, allow_none: true});
//         }
//     }
// }


pub fn focus_release_system(world: &mut World) {
    let mut events: Vec<ReleaseFocusEvent> = Vec::new();
    let mut event_reader = world.resource_mut::<Events<ReleaseFocusEvent>>();
    let mut events_cursor = event_reader.get_cursor();
    // world.run_system_once(|mut event_reader: EventReader<ReleaseFocusEvent>| {
    //
    // }).expect("Could not run focus release event reader system!");

    events = events_cursor.read(&event_reader).map(
        |event| event.clone()
    ).collect::<Vec<_>>();

    //
    // For every event do a recursive focus release if the firing entity is focused.
    for event in events.iter() {
        let current_focus = world.get_resource::<InputFocus>().unwrap().0;
        match event {
            ReleaseFocusEvent::ToParent{entity, allow_none} => {
                if current_focus != Some(*entity) {
                    continue;
                }

                let mut parent_focused = false;
                loop {
                    let mut current_child = *entity;
                    if let Some(policy) = world.get::<InputFocusPolicy>(current_child) {
                        match policy {
                            InputFocusPolicy::None => {
                                continue;
                            }
                            InputFocusPolicy::All => {
                                world.get_resource_mut::<InputFocus>().unwrap().0 = Some(current_child);
                                parent_focused = true;
                                break;
                            }
                        }
                    }

                    if let Some(parent_entity) = world.get::<Parent>(current_child) {
                        current_child = parent_entity.get();
                    } else {
                        break;
                    }
                }

                if !parent_focused && *allow_none {
                    world.get_resource_mut::<InputFocus>().unwrap().0 = None;
                }
            }
            ReleaseFocusEvent::All => {
                if current_focus.is_some() {
                    world.get_resource_mut::<InputFocus>().unwrap().0 = None;
                }
            }
        }
    }

}