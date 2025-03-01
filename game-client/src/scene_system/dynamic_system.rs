// use bevy::prelude::*;

// Systems that can be used with Reflect and the scene system.
// pub trait DynamicSystem {
//     fn run(&self, world: &mut World);
// }

//
// pub trait DynamicSystemRunner {
//     fn run_dynamic_system<T: DynamicSystem>(&mut self, system: T);
// }
//
// impl<'w, 's> DynamicSystemRunner for Commands<'w, 's> {
//     fn run_dynamic_system<T: DynamicSystem>(&mut self, system: T) {
//         self.queue(
//             move |&mut mut world| system.run(&mut world)
//         );
//     }
// }