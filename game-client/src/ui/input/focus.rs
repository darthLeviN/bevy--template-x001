use bevy::prelude::*;
use crate::event_system::{UnhandledEventWorldExt, UnhandledInputEvent};
use crate::ui::input::input_map::*;

pub struct UiFocusPlugin;

impl Plugin for UiFocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(click_focus_change_observer);
        app.add_observer(default_focus_release_input_observer);
        app.add_observer(focus_release_despawn_system);
        app.add_systems(PostUpdate, focus_release_system);
        app.add_systems(PreUpdate, keyboard_event_system);
        // app.add_systems(Last, focus_release_despawn_system);
        app.add_event::<ReleaseFocusEvent>();
        app.register_type::<InputFocusPolicy>();
        app.insert_resource(InputFocus(None));
    }
}

// TODO : add focus changed event and trigger.


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
pub fn click_focus_change_observer(trigger: Trigger<Pointer<Click>>, query: Query<Entity>, _: ResMut<InputFocus>, mut commands: Commands) {
    let trigger_entity = trigger.entity();
    if let Ok(entity) = query.get(trigger_entity) {
        debug!(entity = ?entity, "focus queued");
        commands.queue_next_focus(Some(entity));
    }
}


pub trait QueueNextFocusTrait {
    fn queue_next_focus(&mut self, entity: Option<Entity>);
}

impl<'w, 's> QueueNextFocusTrait for Commands<'w,'s> {
    fn queue_next_focus(&mut self, entity: Option<Entity>) {
        self.queue(move |world: &mut World| {
            _ = world.set_next_input_focus(entity);
        });
    }
}

pub trait SetNextInputFocusTrait {
    fn set_next_input_focus(&mut self, entity: Option<Entity>) -> Result<(), ()>;
}

impl SetNextInputFocusTrait for World {
    fn set_next_input_focus(&mut self, entity: Option<Entity>) -> Result<(), ()>{
        if let Some(entity) = entity {
            if self.get_entity(entity).is_ok() {
                let mut input_focus = self.get_resource_mut::<InputFocus>().unwrap();
                if input_focus.0 != Some(entity) {
                    input_focus.0 = Some(entity);
                    debug!(entity = ?entity, "Focus changed");
                }
                Ok(())
            } else {
                warn!(entity = ?entity, "Focus change failed: Entity not found");
                Err(())
            }
        } else {
            let mut input_focus = self.get_resource_mut::<InputFocus>().unwrap();
            input_focus.0 = None;
            Ok(())
        }
    }
}


pub fn default_focus_release_input_observer(trigger: Trigger<UnhandledInputEvent<MappedInputEvent>>, mut event_writer: EventWriter<ReleaseFocusEvent>) {
    if trigger.event().event.keys.contains(&"ui_focus_release".to_string()) {
        debug!("Focus release key event");
        event_writer.send(ReleaseFocusEvent::ToParent {
            entity: trigger.entity(),
            allow_none: true,
        });
    }
}

pub fn focus_release_despawn_system(
    trigger: Trigger<OnRemove, InputFocusPolicy>,
    // input_focus: Res<InputFocus>,
    parents: Query<&Parent>,
    mut commands: Commands) {
    let entity = trigger.entity();
    let ancestors = parents.iter_ancestors(entity).collect::<Vec<_>>();

    commands.queue(move |world: &mut World| {
        let current_focus = world.get_resource::<InputFocus>().unwrap().0;
        if current_focus != Some(entity) {
            return;
            // Entity already changed by another actor.
        }

        debug!(entity = ?entity, ancestors = ?ancestors, "Focused entity lost. Falling back");

        for entity in ancestors {
            if let Some(policy) = world.get::<InputFocusPolicy>(entity) {
                match policy {
                    InputFocusPolicy::All => {
                        if world.set_next_input_focus(Some(entity)).is_ok() {
                            debug!(entity = ?entity, "Focus fall back");
                            break;
                        };
                    }
                    _ => {}
                }
            }
        }
    });

}


pub fn focus_release_system(world: &mut World) {
    let mut event_reader = world.resource_mut::<Events<ReleaseFocusEvent>>();

    let events = event_reader.drain().map(
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
                    let current_child = *entity;
                    if let Some(policy) = world.get::<InputFocusPolicy>(current_child) {
                        match policy {
                            InputFocusPolicy::None => {
                                continue;
                            }
                            InputFocusPolicy::All => {
                                if world.set_next_input_focus(Some(current_child)).is_ok() {
                                    parent_focused = true;
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(_) = world.get::<Parent>(current_child) {
                        // current_child = parent_entity.get();
                    } else {
                        break;
                    }
                }

                if !parent_focused && *allow_none {
                    _ = world.set_next_input_focus(None)
                }
            }
            ReleaseFocusEvent::All => {
                if current_focus.is_some() {
                    _ = world.set_next_input_focus(None)
                }
            }
        }
    }

}


fn keyboard_event_system(mut event_reader: EventReader<MappedInputEvent>, input_focus: Res<InputFocus>, mut commands: Commands) {
    let mapped_events = event_reader.read().map(|event| event.clone()).collect::<Vec<_>>();
    let focus_entity = input_focus.0;
    commands.queue(move |world: &mut World| {
        for event in mapped_events.into_iter() {
            for event_key in event.keys.iter() {
                debug!(?event_key, ?focus_entity, "Mapped key event");
            }
            world.trigger_unhandled_event(event, focus_entity.unwrap_or(Entity::PLACEHOLDER));
        }
    });


}