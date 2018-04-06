extern crate tiled;

use std::any::TypeId;
use component_states::ComponentStates;
use sprite::Sprite;
use entity_states::EntityStates;
use piston_window::*;
use app::{ButtonStates};
use std::any::Any;
use std::collections::*;
use std::cmp::Eq;
use self::tiled::Map;
use snowflake::ProcessUniqueId;
use component::DestroyType;

impl PartialEq for Entity {
  fn eq(&self, other: &Entity) -> bool {
    self.get_id() == other.get_id()
  }
}

impl Eq for Entity {}

pub trait AsAny {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

#[derive(Debug)]
pub struct Entity {
    id: ProcessUniqueId,
    pub components: ComponentStates,
}

impl Entity {
    pub fn new(components: ComponentStates) -> Entity {
        Entity {
            id: ProcessUniqueId::new(),
            components: components,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        let component_types: Vec<TypeId> = self.components.keys().cloned().collect();
        for type_id in component_types {
            let mut component_result = self.components.remove(&type_id);
            if let Some(mut component) = component_result {
                let delete = component.update(self, args, keys, entities, &map);
                match delete {
                    DestroyType::Component => {},
                    DestroyType::Entity => {return true;},
                    DestroyType::None => {self.components.insert(type_id, component);}
                }
            }
        }

        false
    }

    pub fn draw(&mut self, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        let component_types: Vec<TypeId> = self.components.keys().cloned().collect();
        for type_id in component_types {
            let mut component_result = self.components.remove(&type_id);
            if let Some(mut component) = component_result {
                component.draw(self, event, args, image, context, gl, sprites);
                self.components.insert(type_id, component);
            }
        }
    }

    pub fn get_id(&self) -> ProcessUniqueId {
        self.id
    }

    pub fn get_components(&self) -> &ComponentStates {
        &self.components
    }

    pub fn get_components_mut(&mut self) -> &mut ComponentStates {
        &mut self.components
    }
}
