use std::time::Duration;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use crate::scene_system::{GenericUiSceneCreator, InstantSpawnState, SpawnState, UiSceneCreatorFn};
use crate::ui::ui_navigation::{UiNavigationEvent};

pub struct LoadingPagePlugin;

impl Plugin for LoadingPagePlugin {
    fn build(&self, app: &mut App) {
        _ = app.world_mut().run_system_once(loading_page.get_system());
        app.register_type::<LoadingPageTimer>();
        app.add_systems(Update, loading_page_timer_system);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingPageTimer {
    timer: Timer,
}

fn loading_page_timer_system(mut commands: Commands,time: Res<Time>, mut query: Query<(Entity, &mut LoadingPageTimer)>) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.times_finished_this_tick() > 0 {
            // commands.modify_context(entity, |nav: &mut UiNavigation| {
            //     println!("Loading finished");
            //     nav.next_path = Some(Vec::new());
            // });
            commands.trigger_targets(UiNavigationEvent::SetPath(Vec::new()), entity);
        }
    }
}

fn loading_page(_: &mut World) -> anyhow::Result<GenericUiSceneCreator> {
    let mut new_world: World = World::default();

    new_world.spawn(
        (Text::new("Loading..."),
         LoadingPageTimer {
             timer: Timer::new(Duration::from_secs(1), TimerMode::Once)
         }
        ),
    );


    let scene = Scene::new(new_world);

    return Ok(
        GenericUiSceneCreator{
            scene,
            path: "loading_page".to_string(),
            state: SpawnState::Instant(InstantSpawnState::Loaded),
        }
    )
}