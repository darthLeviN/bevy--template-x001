use std::borrow::Cow;
use crate::ui::interaction::interaction_style::InteractionNodeStyle;
use bevy::prelude::*;

#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct ThemeElement {
    pub class: Cow<'static, str>,
    pub id: Cow<'static, str>,
}

#[derive(Resource, Default)]
pub struct Theme {
    pub class_stylesheet: std::collections::HashMap<Cow<'static, str>, InteractionNodeStyle>,
}

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThemeElement>();
        app.insert_resource(Theme::default());
        app.add_systems(PostUpdate, on_changed_theme_element_system);
    }
}

fn on_changed_theme_element_system(
    mut commands: Commands,
    theme_elements: Res<Theme>,
    query: Query<(Entity, &ThemeElement), Changed<ThemeElement>>,
) {
    for (entity, theme_element) in query.iter() {
        let class_style = theme_elements.class_stylesheet.get(&theme_element.class);
        if let Some(class_style) = class_style {
            commands.entity(entity).insert(class_style.clone());
        } else {
            commands.entity(entity).remove::<InteractionNodeStyle>();
        }
    }
}