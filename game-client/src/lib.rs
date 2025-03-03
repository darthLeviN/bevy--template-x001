#![feature(let_chains)]

pub mod ui;
pub mod scene_system;
pub mod asset_util;

pub mod context_system;
pub mod components;

pub mod event_system;

use bevy::prelude::*;
pub struct StartupPlugins;

impl Plugin for StartupPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultPlugins,))
            .add_plugins(asset_util::AssetLoadingUtilPlugin )
            .add_plugins((scene_system::SceneSystemPlugin,))
            .add_plugins((ui::plugins::UiPlugin,));
    }
}