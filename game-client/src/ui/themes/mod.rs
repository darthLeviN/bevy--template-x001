mod default_theme;

use bevy::prelude::*;
use crate::ui::themes::default_theme::DefaultThemePlugin;

pub struct ThemesPlugin;

impl Plugin for ThemesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultThemePlugin);
    }
}