use bevy::prelude::*;
use crate::scene_system::{GenericUiSceneCreator, InstantSpawnState, SpawnState};

pub struct SettingsPagePlugin;


impl Plugin for SettingsPagePlugin {
    fn build(&self, app: &mut App) {

    }
}

// fn settings(_: &mut World) -> anyhow::Result<GenericUiSceneCreator> {
//     let mut world = World::new();
//
//
//
//
//     Ok(GenericUiSceneCreator {
//         path: "main_menu_page".into(),
//         scene: Scene::new(world),
//         state: SpawnState::Instant(InstantSpawnState::Loaded),
//     })
// }