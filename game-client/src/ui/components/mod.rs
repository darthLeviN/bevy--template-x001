pub mod text_input;
pub mod ui_elements;

use bevy::prelude::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
    }
}

pub const FULL_SIZE_NODE: Node = Node {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),

    display: Display::DEFAULT,
    position_type: PositionType::DEFAULT,
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,
    flex_direction: FlexDirection::DEFAULT,
    flex_wrap: FlexWrap::DEFAULT,
    align_items: AlignItems::Center,
    justify_items: JustifyItems::DEFAULT,
    align_self: AlignSelf::DEFAULT,
    justify_self: JustifySelf::DEFAULT,
    align_content: AlignContent::DEFAULT,
    justify_content: JustifyContent::Center,
    margin: UiRect::DEFAULT,
    padding: UiRect::DEFAULT,
    border: UiRect::DEFAULT,
    flex_grow: 0.0,
    flex_shrink: 1.0,
    flex_basis: Val::Auto,
    min_width: Val::Auto,
    min_height: Val::Auto,
    max_width: Val::Auto,
    max_height: Val::Auto,
    aspect_ratio: None,
    overflow: Overflow::DEFAULT,
    overflow_clip_margin: OverflowClipMargin::DEFAULT,
    row_gap: Val::ZERO,
    column_gap: Val::ZERO,
    grid_auto_flow: GridAutoFlow::DEFAULT,
    grid_template_rows: Vec::new(),
    grid_template_columns: Vec::new(),
    grid_auto_rows: Vec::new(),
    grid_auto_columns: Vec::new(),
    grid_column: GridPlacement::DEFAULT,
    grid_row: GridPlacement::DEFAULT,
};