use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use game_client::scene_system::{SceneMap};
use game_client::StartupPlugins;

fn main() {

    App::new()
        .add_plugins(StartupPlugins)
        .add_systems(Startup, test_startup_plugin)
        .add_systems(Startup, startup_camera)
        .add_systems(Update, test_system)
        .run();
}


fn startup_camera(mut commands: Commands) {
    
    commands.spawn(Camera2d::default());
}


fn test_startup_plugin(mut commands: Commands, scene_map: Res<SceneMap>) {

    let main_pages = scene_map.scenes.get("main_pages").unwrap();
    _ = main_pages.clone().spawn_with_commands(&mut commands).id();
}

fn test_system(query: Query<&TextLayoutInfo>) {
    // for layout_info in query.iter() {
    //     println!("{:?}", layout_info.size);
    // }
}

// #[derive(Component, Reflect)]
// struct TempStruct {
//     pub a: fn()
// }