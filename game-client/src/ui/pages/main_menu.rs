use bevy::color::palettes::basic::*;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use crate::scene_system::{GenericUiSceneCreator, InstantSpawnState, SpawnState, UiSceneCreatorFn};
use crate::ui::components::text_creator::TextCreator;
use crate::ui::interaction::interaction_style::{InteractionNodeStyle, NodeStyleBundle};
use crate::ui::input::focus::InputFocusPolicy;
use crate::ui::theme::ThemeElement;
use crate::ui::ui_navigation::UiNavigationEvent;

pub struct MainMenuPagePlugin;

impl Plugin for MainMenuPagePlugin {
    fn build(&self, app: &mut App) {
        _ = app.world_mut().run_system_once(main_menu.get_system());
    }
}

fn main_menu(_: &mut World) -> anyhow::Result<GenericUiSceneCreator> {
    let mut world = World::new();

    // Button v container
    let vbox = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            padding: UiRect { left: Val::Px(10.0), top: Val::Px(10.0), right: Val::Px(10.0), bottom: Val::Px(10.0)},
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(GREEN.into()),
    );

    world.spawn(vbox)
        .with_children( |parent| {
            // let next_game_button_styles = InteractionNodeStyle {
            //     default_style : NodeStyleBundle {
            //         background_color: BackgroundColor(BLUE.into()),
            //         border_radius: BorderRadius::all(Val::Px(10.0)),
            //         border_color: BorderColor(WHITE.into()),
            //         ..default()
            //     },
            //     hover_style : Some(NodeStyleBundle {
            //         background_color: BackgroundColor(YELLOW.into()),
            //         border_radius: BorderRadius::all(Val::Px(10.0)),
            //         text_color: TextColor(BLACK.into()),
            //         ..default()
            //     }),
            //     ..default()
            // };
            let main_menu_button = (
                Button,
                ThemeElement::from_class("button"),
                // next_game_button_styles,
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                InputFocusPolicy::All
            );

            // let new_game_text = (
            //     Text::from("New Game"),
            //     PickingBehavior::IGNORE,
            // );
            //
            // let options_text = (
            //     Text::from("Options"),
            //     PickingBehavior::IGNORE
            // );

            parent.spawn(
                (
                    main_menu_button.clone(),
                    UiNavigationEvent::AppendPath(vec!["new_game".to_string()]),
                    TextCreator::from("New Game"),
                ));

            parent.spawn((
                main_menu_button.clone(),
                UiNavigationEvent::AppendPath(vec!["options".to_string()]),
                TextCreator::from("Options"),
            ));
        });

    Ok(GenericUiSceneCreator {
        path: "main_menu_page".into(),
        scene: Scene::new(world),
        state: SpawnState::Instant(InstantSpawnState::Loaded),
    })
}
