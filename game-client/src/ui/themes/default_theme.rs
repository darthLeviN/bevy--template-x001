use bevy::color::palettes::basic::{BLACK, BLUE, WHITE, YELLOW};
use bevy::prelude::*;
use crate::ui::interaction::interaction_style::{InteractionNodeStyle, NodeStyleBundle};
use crate::ui::theme::Theme;

pub struct DefaultThemePlugin;

impl Plugin for DefaultThemePlugin {
    fn build(&self, app: &mut App) {
        let mut theme = app.world_mut().resource_mut::<Theme>();
        initialize_default_theme(&mut theme);
    }
}


fn initialize_default_theme(theme: &mut Theme) {
    let button_style = InteractionNodeStyle {
        default_style : NodeStyleBundle {
            background_color: BackgroundColor(BLUE.into()),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            border_color: BorderColor(WHITE.into()),
            ..default()
        },
        hover_style : Some(NodeStyleBundle {
            background_color: BackgroundColor(YELLOW.into()),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            text_color: TextColor(BLACK.into()),
            ..default()
        }),
        ..default()
    };

    theme.class_stylesheet.insert("button".into(), button_style);
}