use bevy::prelude::*;
use crate::scene_system::{GenericUiSceneCreator, InstantSpawnState, SceneMap, SpawnState, UiSceneCreatorFn};
use crate::ui::ui_navigation::{UiNavigation};
use anyhow::Result;
use anyhow::Error;
use bevy::color::palettes::basic::*;
use bevy::ecs::system::RunSystemOnce;
use crate::ui::components::FULL_SIZE_NODE;

pub struct MainPagesNavigationPlugin;

impl Plugin for MainPagesNavigationPlugin {
    fn build(&self, app: &mut App) {
        _ = app.world_mut().run_system_once(main_pages_navigation.get_system());
    }
}

fn main_pages_navigation(base_world: &mut World) -> Result<GenericUiSceneCreator> {
    let mut world = World::new();
    let scene_map = base_world.resource_mut::<SceneMap>();
    let main_menu_scene = scene_map.scenes.get("main_menu_page").ok_or(Error::msg("main_menu scene not found"))?;
    let loading_scene = scene_map.scenes.get("loading_page").ok_or(Error::msg("main_menu scene not found"))?;
    let mut nav = UiNavigation::default();
    nav.root_page = Some(main_menu_scene.clone());
    nav.pages.insert("loading".to_string(),loading_scene.clone());
    nav.next_path = Some(vec!["loading".to_string()]);

    world.spawn(
        (nav,
         FULL_SIZE_NODE.clone(),
         BackgroundColor(BLUE.into()),
        ));

    return Ok(GenericUiSceneCreator {
        path: "main_pages".into(),
        scene: Scene::new(world),
        state: SpawnState::Instant(InstantSpawnState::Loaded),
    })
}