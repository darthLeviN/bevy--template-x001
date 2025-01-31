use bevy::color::palettes::basic::BLUE;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use crate::scene_system::{GenericUiSceneCreator, InstantSpawnState, SpawnState, UiSceneCreatorFn};

pub struct MainMenuPagePlugin;

impl Plugin for MainMenuPagePlugin {
    fn build(&self, app: &mut App) {
        _ = app.world_mut().run_system_once(main_menu.get_system());
    }
}

pub struct MainMenuPage {
    scene: Scene,
}

#[derive(Component, Reflect, Default)]
pub struct DummyCmp;

fn main_menu(_: &mut World) -> anyhow::Result<GenericUiSceneCreator> {
    let mut world = World::new();

    world.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    }, BackgroundColor(BLUE.into())));

    return Ok(GenericUiSceneCreator {
        path: "main_menu_page".into(),
        scene: Scene::new(world),
        state: SpawnState::Instant(InstantSpawnState::Loaded),
    });
}
