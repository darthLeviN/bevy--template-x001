use std::cmp::PartialEq;
use bevy::picking::focus::PickingInteraction;
use bevy::prelude::*;
use crate::context_system::unique_entity_ref::UniqueEntityRefs;
use crate::ui::input::focus::{InputFocus, InputFocusPolicy};

pub struct InteractionStylePlugin;

impl Plugin for InteractionStylePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
            Update,
            (
                interaction_node_style_system,
            )
        ).
            register_type::<InteractionNodeStyle>()
        ;
    }
}

#[derive(Default, Clone, Bundle, Reflect)]
pub struct NodeStyleBundle {
    pub background_color: BackgroundColor,
    pub outline: Outline,
    pub border_radius: BorderRadius,
    pub border_color: BorderColor,
    pub text_color: TextColor,
}

#[derive(Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct InteractionNodeStyle {
    pub default_style: NodeStyleBundle,
    pub hover_style: Option<NodeStyleBundle>,
    pub pressed_style: Option<NodeStyleBundle>,
    pub disabled_style: Option<NodeStyleBundle>,
    pub focus_style: Option<NodeStyleBundle>,
    pub hover_focus_style: Option<NodeStyleBundle>,
    pub pressed_focus_style: Option<NodeStyleBundle>,
}

fn interaction_node_style_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &InteractionNodeStyle, Option<&PickingInteraction>, &InteractionNodeStyle, Option<&Children>, Option<&UniqueEntityRefs>, Option<&InputFocusPolicy>),
        Or<(Changed<PickingInteraction>, Changed<UniqueEntityRefs>)>>,
    input_focus: Res<InputFocus>) {
    for (entity, _, interaction, styles, _, unique_refs, focus_policy) in query.iter_mut() {
        let interaction = if let Some(interaction) = interaction { interaction } else { &PickingInteraction::None };
        if let Some(unique_refs) = unique_refs {
            // Redundant change.
            if !unique_refs.changed.contains("text") {
                continue;
            }
        }

        let final_style =
        if focus_policy == Some(&InputFocusPolicy::DISABLED) {
            styles.disabled_style.clone().unwrap_or(styles.default_style.clone())
        } else {
            match interaction {
                PickingInteraction::None => {
                    if Some(entity) == input_focus.0 {
                        styles.focus_style.clone().unwrap_or(styles.default_style.clone())
                    } else {
                        styles.default_style.clone()
                    }
                }
                PickingInteraction::Hovered => {
                    if Some(entity) == input_focus.0 {
                        styles.hover_focus_style.clone().unwrap_or(styles.hover_style.clone().unwrap_or(styles.default_style.clone()))
                    } else {
                        styles.hover_style.clone().unwrap_or(styles.default_style.clone())
                    }
                }
                PickingInteraction::Pressed => {
                    if Some(entity) == input_focus.0 {
                        styles.pressed_focus_style.clone().unwrap_or(styles.pressed_style.clone().unwrap_or(styles.default_style.clone()))
                    } else {
                        styles.pressed_style.clone().unwrap_or(styles.default_style.clone())
                    }
                }
            }
        };

        if let Some(text_entity) = unique_refs.and_then(|unique_refs| unique_refs.refs.get("text")) {
            commands.entity(*text_entity).insert(final_style.text_color.clone());
        }

        commands.entity(entity).insert(final_style);


    }
}