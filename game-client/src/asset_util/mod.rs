use bevy::app::{App, Plugin};

pub mod asset_loading_util;



pub struct AssetLoadingUtilPlugin;

impl Plugin for AssetLoadingUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (asset_loading_util::AppLoadingStatePlugin,));
    }
}