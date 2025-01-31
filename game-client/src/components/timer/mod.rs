use bevy::ecs::observer::Observers;
use bevy::ecs::system::{BoxedSystem, SystemId};
use bevy::prelude::*;

// #[derive(Component, Reflect)]
// pub struct AutoTimer {
//     pub timer: Timer,
//     #[reflect(ignore)]
//     pub system: Option<SystemId>,
// }
//
// impl AutoTimer {
//
//     // fn with_into_system<'a, M>(commands: &mut Commands<'a, 'a>, timer: Timer, system: impl IntoSystem<(), (), M> + 'static) -> Entity
//     // {
//     //     let system = commands.register_system(system);
//     //     let system_entity = system.clone().entity();
//     //     let entity = commands.spawn(
//     //         Self{ timer, system }
//     //     ).id();
//     //     commands.entity(system_entity).set_parent(entity);
//     //     return entity;
//     // }
// }
//
// pub fn auto_timer_system(mut commands: &mut Commands, time: Res<Time>, mut query: Query<(Entity, &mut AutoTimer)>) {
//     let delta = time.delta();
//     for (entity, mut auto_timer) in query.iter_mut() {
//         auto_timer.timer.tick(delta);
//         let elapsed_count = auto_timer.timer.times_finished_this_tick();
//         if elapsed_count > 0 {
//             for _ in 0..elapsed_count {
//                 if let Some(system) = auto_timer.system {
//                     commands.run_system(system);
//                 }
//
//             }
//             if auto_timer.timer.mode() == TimerMode::Once {
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
// }