use anyhow::Error;
use bevy::color::palettes::basic::{BLUE, WHITE};
use bevy::prelude::*;
use game_client::scene_system::{GenericScene, SceneMap};
use game_client::StartupPlugins;
use game_client::ui::components::FULL_SIZE_NODE;
use game_client::ui::ui_navigation::UiNavigation;

fn main() {

    App::new()
        .add_plugins(StartupPlugins)
        .add_systems(Startup, test_startup_plugin)
        // .add_systems(Update, test_system)
        .add_systems(Startup, startup_camera)
        .run();
}


fn startup_camera(mut commands: Commands) {
    
    commands.spawn(Camera2d::default());
}

fn test_system(navs: Query<&UiNavigation>) {
    for nav in navs.iter() {
        println!("fuck");
    }
}

fn test_startup_plugin(mut commands: Commands, scene_map: Res<SceneMap>) {
    let main_pages = scene_map.scenes.get("main_pages").unwrap();

    main_pages.clone().spawn_with_commands(&mut commands);
}