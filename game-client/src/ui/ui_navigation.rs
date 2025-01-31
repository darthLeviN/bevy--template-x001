use crate::scene_system::{GenericScene, InstantSpawnState, SpawnState};
use crate::ui::components::FULL_SIZE_NODE;
use bevy::ecs::system::SystemId;
use bevy::ecs::world::CommandQueue;
use bevy::prelude::*;
use std::collections::HashMap;
use tuple::OpJoin;

#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiNavigation {
    pub pages: HashMap<String, GenericScene>,
    pub root_page: Option<GenericScene>,
    pub path: Vec<String>,
    pub next_path: Option<Vec<String>>,
    pub current_scene_root: Option<Entity>
}

// type UiNavModifierFn = impl FnOnce(&mut UiNavigation) + Send + Sync;

fn ui_navigation_spawn_system(mut commands: Commands, mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation), Added<UiNavigation>>) {
    for (entity, children, mut nav) in navs.iter_mut() {
        info!("handling page navigation spawn");
        if let Some(next_path) = nav.next_path.take() {
            nav.path = next_path;
        }
        let current_page_system = nav.path.last().and_then(|path| nav.pages.get(path)).or_else(|| nav.root_page.as_ref());
        if let Some(current_page_system) = current_page_system {
            current_page_system.clone().spawn_with_commands(&mut commands).set_parent_in_place(entity);
        } else {
            error!("Page not found!");
        }
    }
}

fn ui_navigation_change_system(mut commands: Commands, mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation), Changed<UiNavigation>>) {

    for (entity, children, mut nav) in navs.iter_mut() {
        info!("handling page navigation");
        if let Some(next_path) = nav.next_path.take() {
            let prev_page_path = nav.path.last();

            let next_page_path = next_path.last();

            if prev_page_path != next_page_path {
                info!("next page path: {}", next_page_path.as_ref().unwrap_or(&&("".to_string())));
                info!("removing previous children");
                if let Some(children) = children {
                    for child_entity in children {
                        let child_entity_clone = child_entity.clone();
                        commands.queue(move |mut world: &mut World| {
                            if let Some(page_state) = world.get_mut::<SpawnState>(child_entity_clone) {
                                match &*page_state {
                                    SpawnState::Instant(state) => {
                                        world.despawn(child_entity_clone);
                                    }
                                }
                            }
                        });
                    }
                }
            }

            let next_page_system =
                next_page_path
                    .and_then(|next| nav.pages.get(next))
                    .or_else(|| nav.root_page.as_ref());

            if let Some(next_page_system) = next_page_system {
                next_page_system.clone().spawn_with_commands(&mut commands).set_parent_in_place(entity);
            }

            nav.path = next_path;
        }
    }

}

impl UiNavigation {
    pub fn get_current_path(&self) -> Option<String> {
        self.path.last().cloned()
    }
}

pub struct PageNavigationPlugin;

impl Plugin for PageNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiNavigation>();
        app.add_systems(PostUpdate, (
            ui_navigation_change_system,
            ui_navigation_spawn_system.before(ui_navigation_change_system)
        ));
    }
}