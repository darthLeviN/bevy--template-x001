mod dynamic_system;

use std::collections::HashMap;
use bevy::prelude::*;
use bevy::asset::Assets;
use bevy::reflect::Typed;
use crate::ui::components::FULL_SIZE_NODE;

#[derive(Clone, Reflect)]
pub enum InstantSpawnState {
    Loaded,
    Unloading
}

#[derive(Component, Clone, Reflect)]
pub enum SpawnState {
    Instant(InstantSpawnState),
}

impl Default for SpawnState {
    fn default() -> Self {
        Self::Instant(InstantSpawnState::Loaded)
    }
}

#[derive(Bundle)]
struct InstantSceneBundle<T: Bundle> {
    scene_root: T,
    state: SpawnState,
}

#[derive(Clone, Reflect)]
pub enum GenericScene {
    Scene((SceneRoot, SpawnState)),
    DynamicScene((DynamicSceneRoot, SpawnState)),
    UiScene((SceneRoot, SpawnState)),
    DynamicUiScene((DynamicSceneRoot, SpawnState)),
}

impl GenericScene {
    pub fn spawn_with_commands<'a>(self, mut commands: &'a mut Commands) -> EntityCommands<'a> {
        match self {
            GenericScene::Scene(bundle) => {
                commands.spawn((bundle))
            },
            GenericScene::DynamicScene(bundle) => {
                commands.spawn(bundle)
            },
            GenericScene::UiScene(bundle) => {
                commands.spawn((bundle, FULL_SIZE_NODE.clone(), PickingBehavior::IGNORE))
            },
            GenericScene::DynamicUiScene(bundle) => {
                commands.spawn((bundle, FULL_SIZE_NODE.clone(), PickingBehavior::IGNORE))
            }
        }
    }
}

pub struct GenericSceneCreator {
    pub path: String,
    pub scene: Scene,
    pub state: SpawnState,
}

pub struct GenericUiSceneCreator {
    pub path: String,
    pub scene: Scene,
    pub state: SpawnState,
}

pub struct DynamicSceneCreator {
    pub path: String,
    pub scene: DynamicScene,
    pub state: SpawnState,
}

pub struct DynamicUiSceneCreator {
    pub path: String,
    pub scene: DynamicScene,
    pub state: SpawnState,
}

pub trait SceneCreatorFn {
    fn get_system(self) -> impl FnMut(&mut World);
}

pub trait UiSceneCreatorFn {
    fn get_system(self) -> impl FnMut(&mut World);
}

pub trait DynamicSceneCreatorFn {
    fn get_system(self) -> impl FnMut(&mut World);
}

pub trait DynamicUiSceneCreatorFn {
    fn get_system(self) -> impl FnMut(&mut World);
}

impl<F> SceneCreatorFn for F
where
    F: Fn(&mut World) -> anyhow::Result<GenericSceneCreator> + 'static,
{
    fn get_system(self) -> impl FnMut(&mut World) + 'static {
        let scene_creator = self; // Move `self` into the closure
        move |world: &mut World| {
            match scene_creator(world) {
                Ok(ret_creator) => {
                    let scene_handle = world.resource_mut::<Assets<Scene>>().add(ret_creator.scene);
                    let creator = GenericScene::Scene((SceneRoot(scene_handle), ret_creator.state));
                    let mut scene_map = world.resource_mut::<SceneMap>();
                    match scene_map.scenes.entry(ret_creator.path.clone()) {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(creator);
                            info!("Scene '{}' added", ret_creator.path);
                        }
                        std::collections::hash_map::Entry::Occupied(_) => {
                            error!("Error: Scene path '{}' already exists", ret_creator.path);
                        }
                    }
                }
                Err(e) => {
                    error!("Error: scene creation failed: {}", e);
                }
            }

        }
    }
}

impl<F> UiSceneCreatorFn for F
where
    F: Fn(&mut World) -> anyhow::Result<GenericUiSceneCreator>,
{
    fn get_system(self) -> impl FnMut(&mut World) {
        let scene_creator = self; // Move `self` into the closure
        move |world: &mut World| {
            match scene_creator(world) {
                Ok(ret_creator) => {
                    let scene_handle = world.resource_mut::<Assets<Scene>>().add(ret_creator.scene);
                    let creator = GenericScene::UiScene((SceneRoot(scene_handle), ret_creator.state));
                    let mut scene_map = world.resource_mut::<SceneMap>();
                    match scene_map.scenes.entry(ret_creator.path.clone()) {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(creator);
                            info!("Ui Scene '{}' added", ret_creator.path);
                        }
                        std::collections::hash_map::Entry::Occupied(_) => {
                            error!("Error: Ui Scene path '{}' already exists", ret_creator.path);
                        }
                    }
                }
                Err(e) => {
                    error!("Error: Ui scene creation failed: {}", e);
                }
            }

        }
    }
}

impl<F> DynamicSceneCreatorFn for F
where
    F: Fn(&mut World) -> anyhow::Result<DynamicSceneCreator> + 'static,
{
    fn get_system(self) -> impl FnMut(&mut World) + 'static {
        let scene_creator = self; // Move `self` into the closure
        move |world: &mut World| {
             match scene_creator(world) {
                 Ok(ret_creator) => {
                     let scene_handle = world.resource_mut::<Assets<DynamicScene>>().add(ret_creator.scene);
                     let creator = GenericScene::DynamicScene((DynamicSceneRoot(scene_handle), ret_creator.state));
                     let mut scene_map = world.resource_mut::<SceneMap>();
                     match scene_map.scenes.entry(ret_creator.path.clone()) {
                         std::collections::hash_map::Entry::Vacant(entry) => {
                             entry.insert(creator);
                             info!("Dynamic scene '{}' added", ret_creator.path);
                         }
                         std::collections::hash_map::Entry::Occupied(_) => {
                             error!("Error: dynamic scene path '{}' already exists", ret_creator.path);
                         }
                     }
                 }
                 Err(e) => {
                     error!("Error: dynamic scene creation failed: {}", e);
                 }
             }
        }
    }
}


impl<F> DynamicUiSceneCreatorFn for F
where
    F: Fn(&mut World) -> anyhow::Result<DynamicUiSceneCreator> + 'static,
{
    fn get_system(self) -> impl FnMut(&mut World) + 'static {
        let scene_creator = self; // Move `self` into the closure
        move |world: &mut World| {
            match scene_creator(world) {
                Ok(ret_creator) => {
                    let scene_handle = world.resource_mut::<Assets<DynamicScene>>().add(ret_creator.scene);
                    let creator = GenericScene::DynamicUiScene((DynamicSceneRoot(scene_handle), ret_creator.state));
                    let mut scene_map = world.resource_mut::<SceneMap>();
                    match scene_map.scenes.entry(ret_creator.path.clone()) {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(creator);
                            info!("Dynamic ui scene '{}' added", ret_creator.path);
                        }
                        std::collections::hash_map::Entry::Occupied(_) => {
                            error!("Error: dynamic ui scene path '{}' already exists", ret_creator.path);
                        }
                    }
                }
                Err(e) => {
                    error!("Error: dynamic scene creation failed: {}", e);
                }
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct SceneMap {
    pub scenes: HashMap<String,GenericScene>
}

pub struct SceneSystemPlugin;

impl Plugin for SceneSystemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpawnState>();
        app.register_type::<InstantSpawnState>();
        app.register_type::<GenericScene>();
        app.insert_resource(
            SceneMap {
                scenes: HashMap::new()
            });
    }
}