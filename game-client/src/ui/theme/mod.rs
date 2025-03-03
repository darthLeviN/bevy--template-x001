use std::borrow::Cow;
use crate::ui::interaction::interaction_style::InteractionNodeStyle;
use bevy::prelude::*;
pub struct ThemeElement {
    pub class: Cow<'static, str>,
    pub id: Cow<'static, str>,
}

pub struct Theme {
    pub class_stylesheet: std::collections::HashMap<Cow<'static, str>, InteractionNodeStyle>,
}

fn on_add_theme_element_system(
    mut commands: Commands,
    theme_elements: Res<Theme>,
    query: Query<ThemeElement, Changed<ThemeElement>>,
) {

}