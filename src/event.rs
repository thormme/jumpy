use std::mem::transmute;
use std::intrinsics::type_name;
use std::any::Any;
use update_event::UpdateEvent;
use std::fmt::Debug;
use entity::AsAny;
use entity::Entity;
use component::Component;
use std::any::TypeId;
use snowflake::ProcessUniqueId;

#[derive(Debug)]
pub struct Event {
    pub event_data: Box<EventData>,
    pub entity: ProcessUniqueId,
    pub component_type: Option<TypeId>,
    pub event_type: TypeId,
}

fn typeid<T: Any>(_: &T) {
    println!("{:?}", TypeId::of::<T>());
}

fn test_type<T>(_: &T) {
    println!("{:?}", unsafe { type_name::<T>() });
}

impl Event {
    pub fn new<E: 'static + EventData>(event_data: E, entity: ProcessUniqueId, component: Option<TypeId>) -> Self {
        Event {
            event_data: Box::new(event_data),
            entity: entity,
            component_type: component,
            event_type: TypeId::of::<E>()
        }
    }

    pub fn get_event_data<E: 'static + EventData>(&self) -> Option<&E> {
        // TODO: Use safe methods
        let e = unsafe{transmute::<&Box<EventData>, &Box<E>>(&self.event_data)};
        Some(e)
    }
}


pub trait EventData : AsAny + Debug {
    fn get_priority(&self) -> u32;
}
