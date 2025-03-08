use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use game_client::scene_system::{SceneMap};
use game_client::StartupPlugins;
use game_client::ui::components::FULL_SIZE_NODE;
use game_client::ui::ui_navigation::UiNavigation;

fn main() {

    App::new()
        .add_plugins(StartupPlugins)
        .add_systems(Startup, test_startup_plugin)
        .add_systems(Startup, startup_camera)
        .add_systems(Update, test_system)
        .add_systems(Update, frame_counter_system)
        .run();
}

#[derive(Component)]
struct FrameCounterElement;

fn frame_counter_system(
    mut query: Query<&mut Text, With<FrameCounterElement>>,
    frame_count: Res<FrameCount>) {
    if let Ok(mut text) = query.get_single_mut() {
        let new_text = format!("Frame counter : {}", frame_count.0);
        // println!("{}", new_text);
        text.0 = new_text;
    }
}


fn startup_camera(mut commands: Commands) {
    
    commands.spawn(Camera2d::default());
}


fn test_startup_plugin(mut commands: Commands, scene_map: Res<SceneMap>) {

    let main_pages = scene_map.scenes.get("main_pages").unwrap();
    let _ = main_pages.clone().spawn_with_commands(&mut commands).id();

    // let node = (Node {
    //     width: Val::Percent(100.0),
    //     height: Val::Percent(100.0),
    //     ..default()
    // }, PickingBehavior::IGNORE);

    // commands.spawn(node).with_children(|parent| {
    //     parent.spawn((Text::from("Hello World!"), TextColor(Color::WHITE), FrameCounterElement));
    // });
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