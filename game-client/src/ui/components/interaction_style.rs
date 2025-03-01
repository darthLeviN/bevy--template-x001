use bevy::picking::focus::PickingInteraction;
use bevy::prelude::*;
use crate::context_system::unique_entity_ref::UniqueEntityRefs;

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
}


fn interaction_node_style_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &InteractionNodeStyle, Option<&PickingInteraction>, &InteractionNodeStyle, Option<&Children>, Option<&UniqueEntityRefs>),
        Or<(Changed<PickingInteraction>, Changed<UniqueEntityRefs>)>,>) {
    for (entity, _, interaction, styles, _, unique_refs) in query.iter_mut() {
        let interaction = if let Some(interaction) = interaction { interaction } else { &PickingInteraction::None };
        if let Some(unique_refs) = unique_refs {
            // Redundant change.
            if !unique_refs.changed.contains("text") {
                continue;
            }
        }

        let final_style = match interaction {
            PickingInteraction::None => {
                styles.default_style.clone()
            }
            PickingInteraction::Hovered => {
                styles.hover_style.clone().unwrap_or(styles.default_style.clone())
            }
            PickingInteraction::Pressed => {
                styles.pressed_style.clone().unwrap_or(styles.default_style.clone())
            }
        };

        if let Some(text_entity) = unique_refs.and_then(|unique_refs| unique_refs.refs.get("text")) {
            commands.entity(*text_entity).insert(final_style.text_color.clone());
        }

        commands.entity(entity).insert(final_style);


    }
}