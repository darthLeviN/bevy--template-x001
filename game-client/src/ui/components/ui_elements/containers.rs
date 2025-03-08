use bevy::prelude::*;
use crate::ui::components::ui_elements::UiElementBuilder;

pub struct VBox {
    row_gap: Val
}

impl From<Val> for VBox {
    fn from(row_gap: Val) -> Self {
        Self {
            row_gap
        }
    }
}

impl Into<Node> for VBox {
    fn into(self) -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            padding: UiRect { left: Val::Px(10.0), top: Val::Px(10.0), right: Val::Px(10.0), bottom: Val::Px(10.0)},
            row_gap: self.row_gap,
            ..default()
        }
    }
}


pub struct MarginContainer {
    padding: UiRect
}

impl From<UiRect> for MarginContainer {
    fn from(padding: UiRect) -> Self {
        Self {
            padding
        }
    }
}

impl Into<Node> for MarginContainer {
    fn into(self) -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: self.padding,
            ..default()
        }
    }
}
