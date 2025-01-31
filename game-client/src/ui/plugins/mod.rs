use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(crate::ui::ui_navigation::PageNavigationPlugin)
            .add_plugins(crate::ui::pages::PagesPlugin)
            .add_plugins(crate::ui::page_navigations::PageNavigationsPlugin);
    }
}