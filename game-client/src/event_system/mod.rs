use std::fmt::Debug;
use bevy::prelude::*;

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, _: &mut App) {
    }
}

pub trait HandledEventExt {
    fn is_handled(&self) -> bool;
    fn set_handled(&mut self, handled: bool);
}

#[derive(Debug, Component)]
pub struct UnhandledInputEvent<T>
where T: Event + Debug + Component
{
    pub(crate) event: T,
}


impl<T> Event for UnhandledInputEvent<T>
where T: Event + Debug + Component
{
    type Traversal = ();

    const AUTO_PROPAGATE: bool = false;
}

pub trait UnhandledEventWorldExt {
    fn trigger_unhandled_event<T: Event + Debug + Component + HandledEventExt>(&mut self, event: T, target: Entity);
}

impl UnhandledEventWorldExt for World {
    fn trigger_unhandled_event<T: Event + Debug + Component + HandledEventExt>(&mut self, mut event: T, target: Entity) {
        if target != Entity::PLACEHOLDER {
            self.trigger_targets_ref(&mut event, target);
        }

        if !event.is_handled() {
            self.trigger_targets(
                UnhandledInputEvent {
                    event,
                },
                target,
            );
        }
    }
}

pub trait UnhandledEventCommandsExt {
    fn trigger_unhandled_event<T: Event + Debug + Component + HandledEventExt>(&mut self, event: T, target: Entity);
}

impl<'w, 's> UnhandledEventCommandsExt for Commands<'w,'s> {
    fn trigger_unhandled_event<T: Event + Debug + Component + HandledEventExt>(&mut self, event: T, target: Entity) {
        self.queue(move |world: &mut World| {
            world.trigger_unhandled_event(event, target);
        })
    }
}

pub trait UnhandledEventTriggerExt {
    fn set_as_handled(&mut self);
}


impl<'w, T> UnhandledEventTriggerExt for Trigger<'w, T>
where
    T: Event + Debug + Component + HandledEventExt,
{
    fn set_as_handled(&mut self) {
        self.propagate(false);
        self.event_mut().set_handled(true);
    }
}