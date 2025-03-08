use bevy::color::palettes::basic::{BLACK, BLUE, WHITE, YELLOW};
use bevy::prelude::*;
use crate::ui::components::ui_elements::containers::MarginContainer;
use crate::ui::interaction::interaction_style::{InteractionNodeStyle, NodeStyleBundle, TextNodeLayout};
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

    // let text_input_style = InteractionNodeStyle {
    //     default_style: NodeStyleBundle {
    //         background_color: BackgroundColor(WHITE.into()),
    //         border_radius: BorderRadius::all(Val::Px(5.0)),
    //         border_color: BorderColor(BLACK.into()),
    //         text_color: TextColor(BLACK.into()),
    //         // padding: UiRect::all(Val::Px(8.0)),
    //         ..default()
    //     },
    //     hover_style: Some(NodeStyleBundle {
    //         background_color: BackgroundColor(BLUE.into()),
    //         border_radius: BorderRadius::all(Val::Px(5.0)),
    //         border_color: BorderColor(YELLOW.into()),
    //         text_color: TextColor(WHITE.into()),
    //         ..default()
    //     }),
    //     focus_style: Some(NodeStyleBundle {
    //         background_color: BackgroundColor(YELLOW.into()),
    //         border_radius: BorderRadius::all(Val::Px(5.0)),
    //         border_color: BorderColor(BLUE.into()),
    //         text_color: TextColor(BLACK.into()),
    //         ..default()
    //     }),
    //     ..default()
    // };

    // theme.class_stylesheet.insert("text_input".into(), text_input_style);

    theme.class_stylesheet.insert("button".into(), button_style);
}