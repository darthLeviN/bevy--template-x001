use std::time::SystemTime;
use bevy::picking::focus::PickingInteraction;
use bevy::prelude::*;
use crate::ui::components::text_creator::TextCreator;
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
        );
        app.register_type::<InteractionNodeStyle>();

        app.add_observer(interaction_node_style_over_observer);
        app.add_observer(interaction_node_style_out_observer);
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


fn determine_final_style(
    entity: Entity,
    interaction: &PickingInteraction,
    styles: &InteractionNodeStyle,
    focus_policy: Option<&InputFocusPolicy>,
    input_focus: &InputFocus,
) -> NodeStyleBundle {
    if focus_policy == Some(&InputFocusPolicy::DISABLED) {
        styles.disabled_style.clone().unwrap_or(styles.default_style.clone())
    } else {
        match interaction {
            PickingInteraction::None => {
                if Some(entity) == input_focus.0 {
                    styles
                        .focus_style
                        .clone()
                        .unwrap_or(styles.default_style.clone())
                } else {
                    styles.default_style.clone()
                }
            }
            PickingInteraction::Hovered => {
                if Some(entity) == input_focus.0 {
                    styles
                        .hover_focus_style
                        .clone()
                        .unwrap_or(styles.hover_style.clone().unwrap_or(styles.default_style.clone()))
                } else {
                    styles.hover_style.clone().unwrap_or(styles.default_style.clone())
                }
            }
            PickingInteraction::Pressed => {
                if Some(entity) == input_focus.0 {
                    styles
                        .pressed_focus_style
                        .clone()
                        .unwrap_or(styles.pressed_style.clone().unwrap_or(styles.default_style.clone()))
                } else {
                    styles.pressed_style.clone().unwrap_or(styles.default_style.clone())
                }
            }
        }
    }
}

fn interaction_node_style_over_observer(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<(Entity, &InteractionNodeStyle, Option<&Children>, Option<&InputFocusPolicy>, Option<&TextCreator>)>,
    input_focus: Res<InputFocus>,
    mut commands: Commands) {
    let picking_interaction = PickingInteraction::Hovered;
    update_interaction_style(trigger.entity(), Some(picking_interaction), &mut query, &mut commands, &input_focus);
}

fn interaction_node_style_out_observer(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(Entity, &InteractionNodeStyle, Option<&Children>, Option<&InputFocusPolicy>, Option<&TextCreator>)>,
    input_focus: Res<InputFocus>,
    mut commands: Commands) {
    let picking_interaction = PickingInteraction::None;
    update_interaction_style(trigger.entity(), Some(picking_interaction), &mut query, &mut commands, &input_focus);
}

fn update_interaction_style(
    entity: Entity,
    picking_interaction: Option<PickingInteraction>,
    mut query: &mut Query<(Entity, &InteractionNodeStyle, Option<&Children>, Option<&InputFocusPolicy>, Option<&TextCreator>)>,
    commands: &mut Commands,
    input_focus: &Res<InputFocus>) {
    if let Ok((entity, interaction_style, children, focus_policy, text_creator)) = query.get(entity) {
        let interaction = if let Some(interaction) = picking_interaction { interaction } else { PickingInteraction::None };
        let final_style = determine_final_style(entity, &interaction, interaction_style, focus_policy, &input_focus);

        commands.entity(entity).insert(final_style);
    }
}


fn interaction_node_style_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &InteractionNodeStyle, Option<&PickingInteraction>, Option<&Children>, Option<&InputFocusPolicy>, Option<&TextCreator>),
        Added<InteractionNodeStyle>>,
    input_focus: Res<InputFocus>) {
    for (entity, interaction_style, picking_interaction, children, focus_policy, text_creator) in query.iter_mut() {
        let now = SystemTime::now();


        let interaction = if let Some(interaction) = picking_interaction { interaction } else { &PickingInteraction::None };
        let final_style = determine_final_style(entity, interaction, interaction_style, focus_policy, &input_focus);

        commands.entity(entity).insert(final_style);
    }
}