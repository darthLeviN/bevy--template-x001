use bevy::prelude::*;

pub mod containers;


pub trait UiElementBuilder {
    fn build(self) -> impl Bundle;
}

pub trait UiElementBuilderWorldExt {
    fn spawn_element(&mut self, element: impl UiElementBuilder) -> EntityWorldMut;
}


impl UiElementBuilderWorldExt for World {
    fn spawn_element(&mut self, element: impl UiElementBuilder) -> EntityWorldMut {
        self.spawn(element.build())
    }
}

pub trait UiElementBuilderCommandsExt {
    fn spawn_element(&mut self, element: impl UiElementBuilder) -> EntityCommands;
}

impl<'w, 's> UiElementBuilderCommandsExt for Commands<'w, 's>  {
    fn spawn_element(&mut self, element: impl UiElementBuilder) -> EntityCommands {
        self.spawn(element.build())
    }
}