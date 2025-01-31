use bevy::prelude::*;
mod main_pages_navigation;

pub struct PageNavigationsPlugin;

impl Plugin for PageNavigationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(main_pages_navigation::MainPagesNavigationPlugin);
    }
}