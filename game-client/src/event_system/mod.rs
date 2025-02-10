use std::fmt::Debug;
use bevy::ecs::component::StorageType;
use bevy::prelude::*;

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Debug, Component)]
pub struct UnhandledEvent<T>
where T: Event + Debug + Component
{
    event: T,
    handled: bool,
}
//
impl<T> Event for UnhandledEvent<T>
where T: Event + Debug + Component
{
    type Traversal = &'static Parent;
    const AUTO_PROPAGATE: bool = true;
}

pub trait UnhandledEventWorldExt {
    fn trigger_unhandled_event<T: Event + Debug + Component>(&mut self, event: T, target: Entity);
}

impl UnhandledEventWorldExt for World {
    fn trigger_unhandled_event<T: Event + Debug + Component>(&mut self, event: T, target: Entity) {
        let mut event = UnhandledEvent {
            event,
            handled: false,
        };

        self.trigger_targets_ref(&mut event, target);

        if !event.handled {
            let mut events = self.get_resource_mut::<Events<T>>().unwrap();
            events.send(event.event);
        }
    }
}

pub trait UnhandledEventCommandsExt {
    fn trigger_unhandled_event<T: Event + Debug + Component>(&mut self, event: T, target: Entity);
}

impl<'w, 's> UnhandledEventCommandsExt for Commands<'w,'s> {
    fn trigger_unhandled_event<T: Event + Debug + Component>(&mut self, event: T, target: Entity) {
        self.queue(move |world: &mut World| {
            world.trigger_unhandled_event(event, target);
        })
    }
}

pub trait UnhandledEventTriggerExt {
    fn set_as_handled(&mut self);
}


impl<'w, T> UnhandledEventTriggerExt for Trigger<'w, UnhandledEvent<T>>
where
    T: Event + Debug + Component,
{
    fn set_as_handled(&mut self) {
        self.propagate(false);
        self.event_mut().handled = true;
    }
}