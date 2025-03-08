use bevy::prelude::*;
use crate::ui::theme::ThemePlugin;
use crate::ui::themes::ThemesPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(crate::ui::ui_navigation::PageNavigationPlugin)
            .add_plugins(crate::ui::pages::PagesPlugin)
            .add_plugins(crate::ui::page_navigations::PageNavigationsPlugin)
            .add_plugins(crate::ui::components::ComponentsPlugin)
            .add_plugins(crate::context_system::ContextSystemPlugin)
            .add_plugins(crate::ui::input::InputPlugin)
            .add_plugins(crate::ui::interaction::CustomInteractionPlugin)
            .add_plugins(ThemePlugin)
            .add_plugins(ThemesPlugin);
    }
}