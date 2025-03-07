use std::borrow::Cow;
use std::time::SystemTime;
use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;


pub struct TextCreatorPlugin;

impl Plugin for TextCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TextCreator>();

        app
            .world_mut()
            .register_component_hooks::<TextCreator>()
            .on_add(on_add_text_creator);

        app.add_systems(PreUpdate, text_creator_style_system);
        app.add_systems(PreUpdate, text_creator_system);

    }
}

#[derive(Component)]
pub struct DerivedText;


#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct TextCreator {
    pub text: Cow<'static, str>,
    pub entity: Entity,
}

impl<T> From<T> for TextCreator
where
    T: Into<Cow<'static, str>>
{
    fn from(value: T) -> Self {
       Self {
           text: value.into(),
           entity: Entity::PLACEHOLDER,
       }
    }
}

impl Default for TextCreator {
    fn default() -> Self {
        Self {
            text: Cow::Borrowed("Default Text"),
            entity: Entity::PLACEHOLDER,
        }
    }
}

fn on_add_text_creator(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let text_creator = world.get::<TextCreator>(entity).unwrap();
    let text = Text::new(text_creator.text.clone());
    let text_entity = world.commands().spawn((text, DerivedText, PickingBehavior::IGNORE)).set_parent_in_place(entity).id();
    let mut text_creator = world.get_mut::<TextCreator>(entity).unwrap();
     text_creator.entity = text_entity;
}

fn text_creator_system(mut commands: Commands, query: Query<&TextCreator, Changed<TextCreator>>) {
    for text_creator in query.iter() {
        commands.entity(text_creator.entity).insert(Text::new(text_creator.text.clone()));
    }
}

fn text_creator_style_system(mut commands: Commands, query: Query<(&TextCreator, &TextColor), Changed<TextColor>>) {
    for (text_creator, text_color) in query.iter() {
        commands.entity(text_creator.entity).insert(text_color.clone());
    }
}