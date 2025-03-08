use std::borrow::Cow;
use bevy::log::tracing_subscriber::filter::combinator::And;
use crate::ui::interaction::interaction_style::InteractionNodeStyle;
use bevy::prelude::*;
use crate::event_system::{HandledEventExt, UnhandledEventCommandsExt, UnhandledEventTriggerExt, UnhandledEventWorldExt};

#[derive(Component, Clone, Reflect, Default, Hash)]
#[reflect(Component)]
pub struct ThemeElement {
    pub class: Cow<'static, str>,
    pub id: Cow<'static, str>,
}

impl ThemeElement {
    pub fn from_class<T: Into<Cow<'static, str>>>(class: T) -> Self {
        Self {
            class: class.into(),
            id: Cow::Borrowed(""),
        }
    }
}


// Default theme will be the resource, the rest will be the component.
#[derive(Resource, Default, Component)]
pub struct Theme {
    pub class_stylesheet: std::collections::HashMap<Cow<'static, str>, InteractionNodeStyle>,
}

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThemeElement>();
        app.insert_resource(Theme::default());
        app.add_systems(PostUpdate, on_changed_theme_system);
    }
}


// Detects change and creates triggers that propagate either up or down to update element.
// Events that propagate down will echo upward. They are theme updates that will require elements to reconfigure.
fn on_changed_theme_system(
    mut commands: Commands,
    theme_query: Query<&Theme>,
    changed_theme_query: Query<Entity, Changed<Theme>>,
    element_query: Query<(&ThemeElement)>,
    changed_element_query: Query<(Entity, &ThemeElement), Changed<ThemeElement>>,
    parent_query: Query<&Parent>,
    children: Query<&Children>,
    default_theme: Res<Theme>) {
    // Algorithm :
    // - Collect changed theme elements into a hashmap
    // - Create a set of entities for changed themes. Ensure subtrees do not intersect by removing repeated parents.
    // - Iterate over the children of the collected theme entities, add theme elements to the initial elements hashmap.
    // - Iterate over ancestors of each theme element, find the style for it.
    use std::collections::HashSet;
    use std::collections::HashMap;

    let theme_entities: HashSet<Entity> = changed_theme_query.iter().collect();

    // First lets collect the elements
    let mut elements: HashMap<Entity, ThemeElement> = HashMap::with_capacity(changed_element_query.iter().count() + theme_entities.len() * 30);

    for (entity, element) in changed_element_query.iter() {
        elements.insert(entity, element.clone());
    }

    for entity in theme_entities.iter() {
        let mut is_unique = true;
        for ancestor in parent_query.iter_ancestors(entity.clone()) {
            if theme_entities.contains(&ancestor) {
                is_unique = false;
                break;
            }
        }
        if is_unique {
            // Does not have a shared parent.
            // Add this entity and it's ThemeElement children to "elements".
            if let Ok(self_element) = element_query.get(entity.clone()) {
                elements.insert(entity.clone(), self_element.clone());
            }

            for descendant in children.iter_descendants(entity.clone()) {
                if let Ok(element) = element_query.get(descendant) {
                    elements.insert(descendant.clone(), element.clone());
                }
            }
        }
    }

    // Now we have a list of all elements that need updating. We will search in their ancestors to see what they have.
    for (entity, element) in elements.iter() {
        // Search through the acnestors. if any is a Theme, try to get the style from it.
        let mut theme_found = false;
        for ancestor in parent_query.iter_ancestors(entity.clone()) {
            if let Ok(theme) = theme_query.get(ancestor) {
                if let Some(style) = theme.class_stylesheet.get(&element.class) {
                    commands.entity(entity.clone()).insert(style.clone());

                    // We managed to find the theme style for this element
                    theme_found = true;
                    break;
                }
            }
        }

        // We didn't find a theme component for it. use the global default theme.
        if !theme_found {
            if let Some(style) = default_theme.class_stylesheet.get(&element.class) {
                commands.entity(entity.clone()).insert(style.clone());
            }
        }
    }

}