use std::cmp::max;
use crate::scene_system::{GenericScene, InstantSpawnState, SpawnState};
use crate::ui::components::FULL_SIZE_NODE;
use bevy::ecs::system::{RunSystemOnce, SystemId};
use bevy::ecs::world::CommandQueue;
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::ecs::component::StorageType;
use tuple::OpJoin;
use crate::event_system::UnhandledEventTriggerExt;
use crate::ui::input::focus::InputFocusPolicy;
use crate::ui::input::input_map::MappedInputEvent;

#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiNavigation {
    pub pages: HashMap<String, GenericScene>,
    pub root_page: Option<GenericScene>,
    pub path: Vec<String>,
    pub next_path: Option<Vec<String>>,

    #[reflect(ignore)]
    pub current_scene_root: Option<Entity>
}

// type UiNavModifierFn = impl FnOnce(&mut UiNavigation) + Send + Sync;

fn ui_navigation_spawn_system(mut commands: Commands, mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation), Added<UiNavigation>>) {
    for (entity, children, mut nav) in navs.iter_mut() {
        let mut observer = Observer::new(ui_navigation_event_observer);
        observer.watch_entity(entity);
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(observer);
        entity_commands.insert(InputFocusPolicy::All);
        info!("handling page navigation spawn");
        if let Some(next_path) = nav.next_path.take() {
            nav.path = next_path;
        }
        let current_page_system = nav.path.last().map_or_else(|| nav.root_page.as_ref(), |path| nav.pages.get(path));
        if let Some(current_page_system) = current_page_system {
            current_page_system.clone().spawn_with_commands(&mut commands).set_parent_in_place(entity);
        } else {
            error!("Page not found!");
        }
    }
}

// TODO : optimize this to use `current_scene_root`
fn ui_navigation_change_system(mut commands: Commands, mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation), Changed<UiNavigation>>) {

    for (entity, children, mut nav) in navs.iter_mut() {
        info!("handling page navigation");
        if let Some(next_path) = nav.next_path.take() {
            let prev_page_path = nav.path.last();

            info!("Changing page path of Entity :{:?} to /{}", entity, next_path.join("/"));

            let next_page_path = next_path.last();

            if prev_page_path != next_page_path {
                info!("next page path: {}", next_page_path.as_ref().unwrap_or(&&("".to_string())));


                if let Some(children) = children {
                    info!("removing previous children : {:?}", children);
                    for child_entity in children {
                        let child_entity_clone = child_entity.clone();
                        commands.queue(move |mut world: &mut World| {
                            if let Some(page_state) = world.get_mut::<SpawnState>(child_entity_clone) {
                                match &*page_state {
                                    SpawnState::Instant(state) => {
                                        if let Ok(entity_commands) = world.get_entity_mut(child_entity_clone) {
                                            entity_commands.despawn_recursive();
                                        }
                                    }
                                }
                            } else {
                                if let Ok(entity_commands) = world.get_entity_mut(child_entity_clone) {
                                    entity_commands.despawn_recursive();
                                }
                            }
                        });
                    }
                }
            }

            // TODO : Add a default page to show invalidation.
            let next_page_system =
                next_page_path
                    .map_or_else(|| nav.root_page.as_ref(), |path| nav.pages.get(path));
                    // .and_then(|next| nav.pages.get(next))
                    // .or_else(|| nav.root_page.as_ref());

            if let Some(next_page_system) = next_page_system {
                next_page_system.clone().spawn_with_commands(&mut commands).set_parent_in_place(entity);
            } else {
                error!("Page not found!");
                commands.spawn(
                    (
                        Text::from("Page not found!"),
                        FULL_SIZE_NODE,
                    )
                ).set_parent_in_place(entity.clone());
                commands.queue(
                    move |mut world: &mut World| {
                        world.run_system_once(move | mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation)>| {
                            if let Ok((ne, children, nav)) = navs.get(entity) {
                                info!("Added page not found text to {:?}. Children: {:?}", ne, children);
                            }
                        }).expect("Could not run page not found text system!");
                    }
                );
            }

            nav.path = next_path;
        }
    }

}

impl UiNavigation {
    pub fn get_current_path(&self) -> Option<String> {
        self.path.last().cloned()
    }

    // pub fn get_current_path_mut(&mut self) -> &mut Vec<String> {
    //     if self.next_path.is_none() {
    //         self.next_path = Some(self.path.clone());
    //     }
    //     self.next_path.as_mut().unwrap()
    // }

    pub fn queue_pop_pages(&mut self, count: usize) {
        if let Some(mut next_path) = self.next_path.as_mut() {
            if !next_path.is_empty() {
                next_path.resize(next_path.len().saturating_sub(count), "".to_string());
            }
        } else {
            if !self.path.is_empty() {
                self.next_path = Some(
                    self.path[..self.path.len().saturating_sub(count)].to_vec()
                )
            }
        }
    }

    pub fn queue_append_page(&mut self, page: String) {
        if page.is_empty() {
            self.queue_pop_pages(usize::MAX);
            return
        }

        // In here we use take. because if next_path exists, we might clear it if we're already on the same page.
        if let Some(mut next_path) = self.next_path.take() {
            if next_path.last() != Some(&page) {
                if self.path.last() == Some(&page) {
                    self.path = next_path;
                    self.path.push(page);
                } else {
                    next_path.push(page);
                    self.next_path = Some(next_path)
                }

            }
        } else {
            if self.path.last() != Some(&page) {
                let mut next_path = self.path.clone();
                next_path.push(page);
                self.next_path = Some(
                    next_path
                );
            }
        }
    }

    pub fn queue_append_path(&mut self, path: &Vec<String>) {
        for page in path {
            self.queue_append_page(page.clone());
        }
    }

    pub fn queue_set_path(&mut self, path: Vec<String>) {
        if let Some(next_path) = self.next_path.take() {
            if next_path.last() != path.last() {
                if self.path.last() == path.last() {
                    self.path = path;
                } else {
                    self.next_path = Some(path);
                }
            } else {
                self.next_path = Some(path);
            }
        } else {
            if self.path.last() == path.last() {
                self.path = path;
            } else {
                self.next_path = Some(path);
            }
        }
    }
}

pub struct PageNavigationPlugin;

impl Plugin for PageNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiNavigation>();
        app.register_type::<UiNavigationEvent>();
        app.add_systems(PostUpdate, (
            ui_navigation_change_system,
            ui_navigation_spawn_system.before(ui_navigation_change_system),
        ));
        app.add_observer(ui_navigation_button_observer);
        app.add_observer(ui_navigation_back_button_observer);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub enum UiNavigationEvent {
    AppendPath(Vec<String>),
    PopPath(usize),
    SetPath(Vec<String>),
}

impl Event for UiNavigationEvent {
    type Traversal = &'static Parent;
    const AUTO_PROPAGATE: bool = true;
}

fn ui_navigation_event_observer(mut trigger: Trigger<UiNavigationEvent>, mut navs: Query<&mut UiNavigation>) {

    if let Ok(mut ui_navigation) = navs.get_mut(trigger.observer()) {
        println!("Found entity");
        match trigger.event_mut() {
            UiNavigationEvent::AppendPath(new_page_stack) => {
                ui_navigation.queue_append_path(new_page_stack);
            }
            UiNavigationEvent::PopPath(amount) => {
                ui_navigation.queue_pop_pages(amount.clone());
            }
            UiNavigationEvent::SetPath(ref mut new_page_stack) => {
                ui_navigation.queue_set_path(new_page_stack.clone());
            }
        }
        trigger.propagate(false);
    }
}

fn ui_navigation_button_observer(mut trigger: Trigger<Pointer<Click>>, mut commands: Commands, query: Query<(Entity, &UiNavigationEvent)>) {
    let entity = trigger.entity();
    info!("button click detected : {:?}", entity);
    if let Ok((entity, navigation_event)) = query.get(entity) {
        info!("Navigation button click detected : {:?}", navigation_event);

        commands.trigger_targets(navigation_event.clone(), entity);
    }

    trigger.propagate(false);
}

fn ui_navigation_back_button_observer(mut trigger: Trigger<MappedInputEvent>, mut commands: Commands, query: Query<(Entity, &UiNavigation)>) {
    if trigger.event().keys.contains(&"ui_focus_release".to_string()) {
        info!("Navigation back button detected : {:?}", trigger.entity());
        let entity = trigger.entity();
        for (entity, navigation) in query.iter() {
            info!("Navigation node : {:?}", entity);
        }
        if let Ok((entity, navigation)) = query.get(entity) {
            info!("Handling navigation back button : {:?}", entity);
            let navigation_event = UiNavigationEvent::PopPath(1);
            commands.trigger_targets(navigation_event.clone(), entity);
            trigger.set_as_handled();
        }
    }
}