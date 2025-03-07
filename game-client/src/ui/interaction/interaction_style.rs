use std::time::SystemTime;
use bevy::core::FrameCount;
use bevy::picking::focus::PickingInteraction;
use bevy::prelude::*;
use crate::ui::components::FULL_SIZE_NODE;
use crate::ui::components::text_creator::TextCreator;
use crate::ui::input::focus::{FocusReleased, Focused, InputFocus, InputFocusPolicy};

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
        app.register_type::<NodeStyle>();

        app.add_observer(interaction_node_style_over_observer);
        app.add_observer(interaction_node_style_out_observer);
        app.add_observer(interaction_node_style_focused_observer);
        app.add_observer(interaction_node_style_released_observer);
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct TextNodeLayout(pub Node);

impl Default for TextNodeLayout {
    fn default() -> Self {
        Self(FULL_SIZE_NODE.clone())
    }
}

#[derive(Default, Clone, Bundle, Reflect)]
pub struct MainStyle {
    pub background_color: BackgroundColor,
    pub outline: Outline,
    pub border_radius: BorderRadius,
    pub border_color: BorderColor,
    pub text_color: TextColor,
    pub text_node_style: TextNodeLayout,
}

#[derive(Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct NodeStyle {
    pub default_style: MainStyle,
    pub hover_style: Option<MainStyle>,
    pub pressed_style: Option<MainStyle>,
    pub disabled_style: Option<MainStyle>,
    pub focus_style: Option<MainStyle>,
    pub hover_focus_style: Option<MainStyle>,
    pub pressed_focus_style: Option<MainStyle>,
}


fn determine_final_style(
    entity: Entity,
    interaction: &PickingInteraction,
    styles: &NodeStyle,
    focus_policy: Option<&InputFocusPolicy>,
    input_focus: &InputFocus,
) -> MainStyle {
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
    mut query: Query<(Entity, &NodeStyle, Option<&Children>, Option<&InputFocusPolicy>)>,
    input_focus: Res<InputFocus>,
    mut commands: Commands) {
    let picking_interaction = PickingInteraction::Hovered;
    update_interaction_style(trigger.entity(), Some(picking_interaction), &mut query, &mut commands, &input_focus);
}


fn interaction_node_style_out_observer(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(Entity, &NodeStyle, Option<&Children>, Option<&InputFocusPolicy>)>,
    input_focus: Res<InputFocus>,
    mut commands: Commands) {
    let picking_interaction = PickingInteraction::None;
    update_interaction_style(trigger.entity(), Some(picking_interaction), &mut query, &mut commands, &input_focus);
}

fn interaction_node_style_focused_observer(
    trigger: Trigger<Focused>,
    mut query: Query<(Entity, &NodeStyle, Option<&Children>, Option<&InputFocusPolicy>)>,
    input_focus: Res<InputFocus>,
    mut commands: Commands
) {
    let picking_interaction = PickingInteraction::None;
    update_interaction_style(trigger.entity(), Some(picking_interaction), &mut query, &mut commands, &input_focus);
}

fn interaction_node_style_released_observer(
    trigger: Trigger<FocusReleased>,
    mut query: Query<(Entity, &NodeStyle, Option<&Children>, Option<&InputFocusPolicy>)>,
    input_focus: Res<InputFocus>,
    mut commands: Commands
) {
    let picking_interaction = PickingInteraction::None;
    update_interaction_style(trigger.entity(), Some(picking_interaction), &mut query, &mut commands, &input_focus);
}

fn update_interaction_style(
    entity: Entity,
    picking_interaction: Option<PickingInteraction>,
    mut query: &mut Query<(Entity, &NodeStyle, Option<&Children>, Option<&InputFocusPolicy>)>,
    commands: &mut Commands,
    input_focus: &Res<InputFocus>) {
    if let Ok((entity, interaction_style, children, focus_policy)) = query.get(entity) {
        let interaction = if let Some(interaction) = picking_interaction { interaction } else { PickingInteraction::None };
        let final_style = determine_final_style(entity, &interaction, interaction_style, focus_policy, &input_focus);

        commands.entity(entity).insert(final_style);
    }
}


fn interaction_node_style_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &NodeStyle, Option<&PickingInteraction>, Option<&Children>, Option<&InputFocusPolicy>, Option<&TextCreator>),
        Added<NodeStyle>>,
    input_focus: Res<InputFocus>) {
    for (entity, interaction_style, picking_interaction, children, focus_policy, text_creator) in query.iter_mut() {
        let now = SystemTime::now();


        let interaction = if let Some(interaction) = picking_interaction { interaction } else { &PickingInteraction::None };
        let final_style = determine_final_style(entity, interaction, interaction_style, focus_policy, &input_focus);

        commands.entity(entity).insert(final_style);
    }
}