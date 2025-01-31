use std::process::Command;
use bevy::prelude::*;

pub trait ContextCommands
{

    fn modify_context<F, C>(&mut self, entity: Entity, modifier_fn: F)
    where
        F: FnOnce(&mut C) + Send + Sync + 'static,
        C: Component
    ;

    fn process_context<C, FP, FW, R>(&mut self, entity: Entity, process_fn: FP, world_fn: FW)
    where
        FP: FnOnce(Entity, &mut C) -> R + Send + Sync + 'static,
        FW: FnOnce(&mut World, R) + Send + Sync + 'static,
        C: Component;

}

impl<'w, 's> ContextCommands for Commands<'w, 's> {
    fn modify_context<F, C>(&mut self, entity: Entity, modifier_fn: F)
    where
        F: FnOnce(&mut C) + Send + Sync + 'static,
        C: Component {
        self.queue(move |world: &mut World| {
            let mut next_child = entity;
            loop {
                let next_parent = world.get::<Parent>(next_child);
                match next_parent {
                    Some(next_parent) => {
                        let next_parent = next_parent.get();
                        let mut nav = world.get_mut::<C>(next_parent.clone());
                        match nav {
                            Some(mut nav) => {
                                modifier_fn(&mut *nav);
                                return;
                            }
                            None => { next_child = next_parent; }
                        }
                    }
                    None => break
                }
            }
        });
    }

    fn process_context<C, FP, FW, R>(&mut self, entity: Entity, process_fn: FP, world_fn: FW)
    where
        FP: FnOnce(Entity, &mut C) -> R + Send + Sync + 'static,
        FW: FnOnce(&mut World, R) + Send + Sync + 'static,
        C: Component {
        self.queue(move |world: &mut World| {
            let mut next_child = entity;
            loop {
                let next_parent = world.get::<Parent>(next_child);
                match next_parent {
                    Some(next_parent) => {
                        let next_parent = next_parent.get();
                        let mut nav = world.get_mut::<C>(next_parent.clone());
                        match nav {
                            Some(mut nav) => {
                                let ret = process_fn(next_parent, &mut *nav);
                                world_fn(world, ret);
                                return;
                            }
                            None => { next_child = next_parent; }
                        }
                    }
                    None => break
                }
            }
        })
        }
}
