
use splitmut::GetMuts;
use std::marker::PhantomData;
use std::any::Any;
use component::Component;
use std::any::TypeId;
use std::collections::hash_map::Keys;
use std::collections::HashSet;
use nalgebra::Point2;
use std::collections::HashMap;
use entity::Entity;
use snowflake::ProcessUniqueId;
use splitmut::{SplitMut, SplitMutError};

pub type ComponentStates = HashMap<TypeId, Box<Component>>;

pub trait ComponentHashMap {
    fn remove_component<T: 'static +  Component>(&mut self) -> Option<T>;

    fn insert_component<T: 'static +  Component>(&mut self, component: T);

    fn get_component<T: 'static +  Component>(&self) -> Option<&T>;

    fn get_mut_component<T: 'static +  Component>(&mut self) -> Option<&mut T>;
}

impl ComponentHashMap for ComponentStates {
    fn remove_component<T: 'static +  Component>(&mut self) -> Option<T> {
        if let Some(component) = self.remove(&TypeId::of::<T>()) {
            let any_component: Box<Any> = Box::new(component);
            if let Ok(component_type) = any_component.downcast::<T>() {
                return Some(*component_type);
            }
        }
        None
    }

    fn insert_component<T: 'static +  Component>(&mut self, component: T) {
        self.insert(TypeId::of::<T>(), Box::new(component));
    }

    fn get_component<T: 'static +  Component>(&self) -> Option<&T> {
        if let Some(component) = self.get(&TypeId::of::<T>()) {
            if let Some(component_type) = component.as_any().downcast_ref::<T>() {
                return Some(component_type);
            }
        }
        None
    }

    fn get_mut_component<T: 'static +  Component>(&mut self) -> Option<&mut T> {
        //let m = self.components.get_muts();
        if let Some(component) = self.get_mut(&TypeId::of::<T>()) {
            if let Some(component_type) = component.as_any_mut().downcast_mut::<T>() {
                return Some(component_type);
            }
        }
        None
    }
}

pub trait MapByTypeId<'a> {
    fn get_mut<T: 'static + Component>(&mut self) -> Option<&'a mut T>;
}

impl<'a> MapByTypeId<'a> for GetMuts<'a, &'a TypeId, Box<Component>, HashMap<TypeId, Box<Component>>> {
    fn get_mut<T: 'static + Component>(&mut self) -> Option<&'a mut T> {
        if let Ok(component) = self.at(&TypeId::of::<T>()) {
            if let Some(component_type) = component.as_any_mut().downcast_mut::<T>() {
                return Some(component_type);
            }
        }
        None
    }
}
