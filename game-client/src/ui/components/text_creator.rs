use std::borrow::Cow;
use std::time::SystemTime;
use bevy::color::palettes::css::*;
use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use crate::ui::components::FULL_SIZE_NODE;
use crate::ui::interaction::interaction_style::TextNodeLayout;

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

// Indicates that the selected text is dervied from a text creator.
#[derive(Component)]
pub struct DerivedText;


#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct TextCreator {
    pub text: Cow<'static, str>,
    pub entity: Entity,
    proxy: Entity, // This is a workaround for a bug where text with direct text child confuses size
    layout_entity: Entity, // This is a workaround for a bug where text with direct text child confuses size
}

impl<T> From<T> for TextCreator
where
    T: Into<Cow<'static, str>>
{
    fn from(value: T) -> Self {
       Self {
           text: value.into(),
           entity: Entity::PLACEHOLDER,
           proxy: Entity::PLACEHOLDER,
           layout_entity: Entity::PLACEHOLDER,
       }
    }
}

impl Default for TextCreator {
    fn default() -> Self {
        Self {
            text: Cow::Borrowed(""),
            entity: Entity::PLACEHOLDER,
            proxy: Entity::PLACEHOLDER,
            layout_entity: Entity::PLACEHOLDER,
        }
    }
}

fn on_add_text_creator(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let text_creator = world.get::<TextCreator>(entity).unwrap();
    println!("spawning text from text creator : {}", text_creator.text);
    let text = Text::new(text_creator.text.clone());
    let mut commands = world.commands();
    // Add dummy text to maintain height
    let mut proxy_entity = commands.spawn(
        (
            Node {
                width: Val::Px(0.0),
                ..default()
            }
            , PickingBehavior::IGNORE, BackgroundColor(BLUE.into())));
    proxy_entity.set_parent_in_place(entity);
    let proxy_entity = proxy_entity.id();
    let layout_entity = commands.spawn((Text::new(" "), PickingBehavior::IGNORE)).set_parent_in_place(proxy_entity).id();
    let text_entity = commands.spawn((text, DerivedText, PickingBehavior::IGNORE)).set_parent_in_place(entity).id();
    let mut text_creator = world.get_mut::<TextCreator>(entity).unwrap();
    text_creator.entity = text_entity;
    text_creator.proxy = proxy_entity;
    text_creator.layout_entity = layout_entity;


}

fn text_creator_system(mut commands: Commands, query: Query<&TextCreator, Changed<TextCreator>>) {
    for text_creator in query.iter() {
        commands.entity(text_creator.entity).insert(Text::new(text_creator.text.clone()));
    }
}

fn text_creator_style_system(
    mut commands: Commands,
    color_query: Query<(&TextCreator, &TextColor), Changed<TextColor>>,
    layout_query: Query<(&TextCreator, &TextNodeLayout), Changed<TextNodeLayout>>,) {
    for (text_creator, text_color) in color_query.iter() {
        commands.entity(text_creator.entity).insert(text_color.clone());
    }

    for (text_creator, text_node_layout) in layout_query.iter() {
        commands.entity(text_creator.entity).insert(text_node_layout.0.clone());
        commands.entity(text_creator.layout_entity).insert(text_node_layout.0.clone());
    }
}