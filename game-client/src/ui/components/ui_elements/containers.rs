use bevy::prelude::*;
use crate::ui::components::ui_elements::UiElementBuilder;

pub struct VBox {
    row_gap: Val
}

impl UiElementBuilder for VBox {
    fn build(self) -> impl Bundle{
        (Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            padding: UiRect { left: Val::Px(10.0), top: Val::Px(10.0), right: Val::Px(10.0), bottom: Val::Px(10.0)},
            row_gap: self.row_gap,
            ..default()
        }, )
    }
}