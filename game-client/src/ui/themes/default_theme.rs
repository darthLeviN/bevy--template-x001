use bevy::color::palettes::basic::{BLACK, BLUE, WHITE, YELLOW};
use bevy::prelude::*;
use crate::ui::components::ui_elements::containers::MarginContainer;
use crate::ui::interaction::interaction_style::{NodeStyle, MainStyle, TextNodeLayout};
use crate::ui::theme::Theme;

pub struct DefaultThemePlugin;

impl Plugin for DefaultThemePlugin {
    fn build(&self, app: &mut App) {
        let mut theme = app.world_mut().resource_mut::<Theme>();
        initialize_default_theme(&mut theme);
    }
}


fn initialize_default_theme(theme: &mut Theme) {
    let button_style = NodeStyle {
        default_style : MainStyle {
            background_color: BackgroundColor(BLUE.into()),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            border_color: BorderColor(WHITE.into()),
            ..default()
        },
        hover_style : Some(MainStyle {
            background_color: BackgroundColor(YELLOW.into()),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            text_color: TextColor(BLACK.into()),
            ..default()
        }),
        ..default()
    };
    theme.class_stylesheet.insert("button".into(), button_style);

    let text_input_text_layout = TextNodeLayout (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        }
    );
    let text_input_style = NodeStyle {
        default_style: MainStyle {
            background_color: BackgroundColor(Color::srgb(0.8, 0.8, 0.8).into()),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            border_color: BorderColor(Color::srgb(0.2, 0.2, 0.2).into()),
            text_color: TextColor(BLACK.into()),
            text_node_style: text_input_text_layout.clone(),
            ..default()
        },
        hover_style: Some(MainStyle {
            background_color: BackgroundColor(Color::srgb(0.7, 0.7, 0.7).into()),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            border_color: BorderColor(Color::srgb(0.4, 0.4, 0.4).into()),
            text_color: TextColor(WHITE.into()),
            text_node_style: text_input_text_layout.clone(),
            ..default()
        }),
        focus_style: Some(MainStyle {
            background_color: BackgroundColor(Color::srgb(0.9, 0.9, 0.9).into()),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            border_color: BorderColor(BLACK.into()),
            text_color: TextColor(BLACK.into()),
            text_node_style: text_input_text_layout.clone(),
            ..default()
        }),
        ..default()
    };

    theme.class_stylesheet.insert("text_input".into(), text_input_style);

}