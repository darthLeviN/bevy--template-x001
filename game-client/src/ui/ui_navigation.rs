use std::collections::HashMap;
use crate::scene_system::{GenericScene, SpawnState};
use crate::ui::components::FULL_SIZE_NODE;
use crate::event_system::UnhandledEventTriggerExt;
use crate::ui::input::focus::InputFocusPolicy;
use crate::ui::input::input_map::MappedInputEvent;
use bevy::ecs::system::{RunSystemOnce};
use bevy::prelude::*;

// Plugin definition
pub struct PageNavigationPlugin;

impl Plugin for PageNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiNavigation>();
        app.register_type::<UiNavigationEvent>();

        app.add_observer(ui_navigation_spawn_observer);
        app.add_observer(ui_navigation_button_observer);
        app.add_observer(ui_navigation_back_button_observer);

        app.add_systems(PostUpdate, (ui_navigation_change_system,));
    }
}

// UiNavigation Component
#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiNavigation {
    pub pages: HashMap<String, GenericScene>,
    pub root_page: Option<GenericScene>,
    pub path: Vec<String>,
    pub next_path: Option<Vec<String>>,

    #[reflect(ignore)]
    pub current_scene_root: Option<Entity>,
}

impl UiNavigation {
    pub fn get_current_path(&self) -> Option<String> {
        self.path.last().cloned()
    }

    pub fn queue_pop_pages(&mut self, count: usize) {
        if let Some(next_path) = self.next_path.as_mut() {
            if !next_path.is_empty() {
                next_path.resize(next_path.len().saturating_sub(count), "".to_string());
            }
        } else if !self.path.is_empty() {
            self.next_path = Some(self.path[..self.path.len().saturating_sub(count)].to_vec());
        }
    }

    pub fn queue_append_page(&mut self, page: String) {
        if page.is_empty() {
            self.queue_pop_pages(usize::MAX);
            return;
        }

        if let Some(mut next_path) = self.next_path.take() {
            if next_path.last() != Some(&page) {
                if self.path.last() == Some(&page) {
                    self.path = next_path;
                    self.path.push(page);
                } else {
                    next_path.push(page);
                    self.next_path = Some(next_path);
                }
            }
        } else if self.path.last() != Some(&page) {
            let mut next_path = self.path.clone();
            next_path.push(page);
            self.next_path = Some(next_path);
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
        } else if self.path.last() == path.last() {
            self.path = path;
        } else {
            self.next_path = Some(path);
        }
    }
}

// UiNavigationEvent Component
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

// Observers
fn ui_navigation_spawn_observer(
    trigger: Trigger<OnAdd, UiNavigation>,
    mut commands: Commands,
    mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation)>,
) {
    let entity = trigger.entity();
    if let Ok((entity, _, mut nav)) = navs.get_mut(entity) {
        let mut observer = Observer::new(ui_navigation_event_observer);
        observer.watch_entity(entity);
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(observer);
        entity_commands.insert(InputFocusPolicy::All);

        debug!("handling page navigation spawn");
        if let Some(next_path) = nav.next_path.take() {
            nav.path = next_path;
        }
        let current_page_system = nav
            .path
            .last()
            .map_or_else(|| nav.root_page.as_ref(), |path| nav.pages.get(path));
        if let Some(current_page_system) = current_page_system {
            current_page_system
                .clone()
                .spawn_with_commands(&mut commands)
                .set_parent_in_place(entity);
        } else {
            error!(?nav.path, "Page scene not found!");
        }
    }
}

fn ui_navigation_change_system(
    mut commands: Commands,
    mut navs: Query<(Entity, Option<&Children>, &mut UiNavigation), Changed<UiNavigation>>,
) {
    for (entity, children, mut nav) in navs.iter_mut() {
        debug!("handling page navigation");
        if let Some(next_path) = nav.next_path.take() {
            let prev_page_path = nav.path.last();

            info!(
                current_path = ?nav.path.join("/"),
                next_path = ?next_path.join("/"),
                "Navigating to the next page"
            );

            let next_page_path = next_path.last();

            if prev_page_path != next_page_path {
                debug!(
                    path = ?next_page_path.as_ref().unwrap_or(&&("".to_string())),
                    "next page path",

                );

                if let Some(children) = children {
                    debug!(?children, "removing previous children");
                    for child_entity in children {
                        let child_entity_clone = *child_entity;
                        commands.queue(move |world: &mut World| {
                            if let Some(page_state) = world.get_mut::<SpawnState>(child_entity_clone)
                            {
                                match &*page_state {
                                    SpawnState::Instant(_) => {
                                        if let Ok(entity_commands) =
                                            world.get_entity_mut(child_entity_clone)
                                        {
                                            entity_commands.despawn_recursive();
                                        }
                                    }
                                }
                            } else if let Ok(entity_commands) =
                                world.get_entity_mut(child_entity_clone)
                            {
                                entity_commands.despawn_recursive();
                            }
                        });
                    }
                }
            }

            let next_page_system = next_page_path
                .map_or_else(|| nav.root_page.as_ref(), |path| nav.pages.get(path));

            if let Some(next_page_system) = next_page_system {
                next_page_system
                    .clone()
                    .spawn_with_commands(&mut commands)
                    .set_parent_in_place(entity);
            } else {
                error!("Page not found!");
                commands
                    .spawn((Text::from("Page not found!"), FULL_SIZE_NODE))
                    .set_parent_in_place(entity.clone());
                commands.queue(move |world: &mut World| {
                    world
                        .run_system_once(
                            move |navs: Query<(Entity, Option<&Children>, &mut UiNavigation)>| {
                                if let Ok((ne, _, _)) = navs.get(entity) {
                                    info!(
                                        error = ?"page_not_found",
                                        entity = ?ne,
                                        "Added Error page"
                                    );
                                }
                            },
                        )
                        .expect("Could not run page not found text system!");
                });
            }

            nav.path = next_path;
        }
    }
}

fn ui_navigation_event_observer(
    mut trigger: Trigger<UiNavigationEvent>,
    mut navs: Query<&mut UiNavigation>,
) {
    if let Ok(mut ui_navigation) = navs.get_mut(trigger.observer()) {
        match trigger.event_mut() {
            UiNavigationEvent::AppendPath(new_page_stack) => {
                ui_navigation.queue_append_path(new_page_stack);
            }
            UiNavigationEvent::PopPath(amount) => {
                ui_navigation.queue_pop_pages(*amount);
            }
            UiNavigationEvent::SetPath(ref mut new_page_stack) => {
                ui_navigation.queue_set_path(new_page_stack.clone());
            }
        }
        trigger.propagate(false);
    }
}

fn ui_navigation_button_observer(
    mut trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    query: Query<(Entity, &UiNavigationEvent)>,
) {
    let entity = trigger.entity();

    if let Ok((entity, navigation_event)) = query.get(entity) {
        info!(?navigation_event, "Navigation button click detected");
        commands.trigger_targets(navigation_event.clone(), entity);
    }

    trigger.propagate(false);
}

fn ui_navigation_back_button_observer(
    mut trigger: Trigger<MappedInputEvent>,
    mut commands: Commands,
    query: Query<(Entity, &UiNavigation)>,
) {
    if trigger.event().keys.contains(&"ui_focus_release".to_string()) {
        info!(entity = ?trigger.entity(), "Navigation back button detected");
        let entity = trigger.entity();

        for (entity, _) in query.iter() {
            debug!(?entity, "Navigation node");
        }

        if let Ok((entity, _)) = query.get(entity) {
            debug!(?entity, "Handling navigation back button");
            let navigation_event = UiNavigationEvent::PopPath(1);
            commands.trigger_targets(navigation_event.clone(), entity);
            trigger.set_as_handled();
        }
    }
}