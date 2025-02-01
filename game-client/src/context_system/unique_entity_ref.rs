use std::collections::{HashMap, HashSet};
use bevy::asset::AssetContainer;
use bevy::prelude::*;

pub struct UniqueEntityRefPlugin;

impl Plugin for UniqueEntityRefPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(
                Last,
                unique_entity_ref_system,
            );
        app.register_type::<UniqueEntity>();
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UniqueEntity {
    pub tag: &'static str
}

#[derive(Component, Clone)]
pub struct UniqueEntityParentRef {
    pub entity: Entity,
}

#[derive(Default, Component, Clone)]
pub struct UniqueEntityRefs {
    pub refs: HashMap<&'static str, Entity>,
    pub changed: HashSet<&'static str>
}

//
fn unique_entity_ref_system(
    mut commands: Commands,
    query: Query<(Entity, &UniqueEntity, Option<&Parent>, Option<&UniqueEntityParentRef>), Changed<Parent>>) {
    let mut update: Vec<(Entity, &'static str, Option<Entity>, Option<Entity>)> =
        query.iter().map(|(entity, unique_entity, parent, prev_parent)| {
            (
                entity,
                unique_entity.tag,
                parent.map(|p| p.get()),
                prev_parent.map(|p| p.entity)
            )
        }).collect();

    commands.queue(move |world: &mut World| {
        // Clear `changed` first.
        for (entity, unique_entity, parent, prev_parent) in update.iter() {
            if let Some(mut unique_refs) = world.entity_mut(*entity).get_mut::<UniqueEntityRefs>() {
                unique_refs.changed.clear();
            }
        }

        for (entity, unique_entity, parent, prev_parent) in update.into_iter() {
            if let Some(prev_parent) = prev_parent {
                let mut parent_mut = world.entity_mut(prev_parent.clone());
                if let Some(mut next_uniques) = parent_mut.get_mut::<UniqueEntityRefs>() {
                    next_uniques.refs.remove(unique_entity);
                    next_uniques.changed.insert(unique_entity);
                }
            }

            if let Some(parent) = parent {
                let mut parent_mut = world.entity_mut(parent.clone());
                if let Some(mut next_uniques) = parent_mut.get_mut::<UniqueEntityRefs>() {
                    next_uniques.refs.insert(unique_entity, entity);
                    next_uniques.changed.insert(unique_entity);
                } else {
                    let mut new_refs = UniqueEntityRefs::default();
                    new_refs.refs.insert(unique_entity, entity);
                    new_refs.changed.insert(unique_entity);
                    parent_mut.insert(new_refs);
                }

                world.entity_mut(entity).insert(UniqueEntityParentRef { entity: parent });
            } else {
                world.entity_mut(entity).remove::<UniqueEntityParentRef>();
            }
        }
    });

}