pub mod main_menu;
mod loading_page;

pub struct PagesPlugin;

use bevy::prelude::*;
use crate::ui::pages::loading_page::LoadingPagePlugin;
use crate::ui::pages::main_menu::MainMenuPagePlugin;

impl Plugin for PagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                MainMenuPagePlugin,
                LoadingPagePlugin,));
    }
}