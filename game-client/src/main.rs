use bevy::prelude::*;
use game_client::scene_system::{SceneMap};
use game_client::StartupPlugins;

fn main() {

    App::new()
        .add_plugins(StartupPlugins)
        .add_systems(Startup, test_startup_plugin)
        .add_systems(Startup, startup_camera)
        .run();
}


fn startup_camera(mut commands: Commands) {
    
    commands.spawn(Camera2d::default());
}


fn test_startup_plugin(mut commands: Commands, scene_map: Res<SceneMap>) {

    let main_pages = scene_map.scenes.get("main_pages").unwrap();
    // let mut observer = Observer::new(|trigger: Trigger<UiNavigationEvent>| {
    // });
    //
    // let mut observer2 = Observer::new(|trigger: Trigger<UiNavigationEvent>, query: Query<&Parent>| {
    // });

    // commands.spawn(observer2);

    _ = main_pages.clone().spawn_with_commands(&mut commands).id();//.insert(observer);
    // let empty_entity = commands.spawn_empty().id();
    // observer.watch_entity(nav_id);
    // commands.spawn(observer);


    // let tempworld = World::default();

}

// #[derive(Component, Reflect)]
// struct TempStruct {
//     pub a: fn()
// }