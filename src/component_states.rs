
use std::any::Any;
use component::Component;
use std::any::TypeId;
use std::collections::hash_map::Keys;
use std::collections::HashSet;
use nalgebra::Point2;
use std::collections::HashMap;
use entity::Entity;
use snowflake::ProcessUniqueId;

#[derive(Debug)]
pub struct ComponentStates {
    components: HashMap<TypeId, Box<Component>>,
}

impl ComponentStates {
    pub fn new() -> ComponentStates {
        ComponentStates {
            components: HashMap::new(),
        }
    }

    pub fn remove<T: 'static +  Component>(&mut self) -> Option<T> {
        if let Some(component) = self.components.remove(&TypeId::of::<T>()) {
            let any_component: Box<Any> = Box::new(component);
            if let Ok(component_type) = any_component.downcast::<T>() {
                return Some(*component_type);
            }
        }
        None
    }

    pub fn remove_component(&mut self, type_id: TypeId) -> Option<Box<Component>> {
        if let Some(component) = self.components.remove(&type_id) {
            return Some(component);
        }
        None
    }

    pub fn insert<T: 'static +  Component>(&mut self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn insert_component(&mut self, type_id: TypeId, component: Box<Component>) {
        self.components.insert(type_id, component);
    }

    pub fn keys(&self) -> Keys<TypeId, Box<Component>> {
        self.components.keys()
    }

    pub fn get<T: 'static +  Component>(&self) -> Option<&T> {
        if let Some(component) = self.components.get(&TypeId::of::<T>()) {
            if let Some(component_type) = component.as_any().downcast_ref::<T>() {
                return Some(component_type);
            }
        }
        None
    }

    pub fn get_mut<T: 'static +  Component>(&mut self) -> Option<&mut T> {
        if let Some(component) = self.components.get_mut(&TypeId::of::<T>()) {
            if let Some(component_type) = component.as_any_mut().downcast_mut::<T>() {
                return Some(component_type);
            }
        }
        None
    }
}
