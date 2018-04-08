use event::EventData;
use event::Event;
use app::{EventMap, EventState};
use std::any::TypeId;
use std::collections::HashMap;
use component::DestroyType;
use entity_states::EntityStates;
use app::ButtonStates;
use piston_window::UpdateArgs;
use entity::Entity;
use component::Component;
use snowflake::ProcessUniqueId;
use tiled::Map;
use update_event;
use event;

#[derive(Debug)]
pub struct Damageable {
    health: u32,
    invulnerable_default: f64,
    invulnerable_timer: f64,
}

#[derive(Debug)]
pub struct DamageEvent {
    pub entity_id: ProcessUniqueId,
    pub amount: i32,
}

impl EventData for DamageEvent {
    fn get_priority(&self) -> u32 {
        50u32
    }
}

impl Component for Damageable {
    fn handle_event(&mut self, event_type: TypeId, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        if event_type == TypeId::of::<update_event::UpdateEvent>() {
            self.update(event, entity, keys, entities, map, events)
        } else {
            DestroyType::None
        }
    }
}

impl Damageable {
    pub fn new(health: u32, invulnerable_timeout: f64) -> Self {
        Damageable {
            health: health,
            invulnerable_default: invulnerable_timeout,
            invulnerable_timer: 0f64,
        }
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn set_health(&mut self, new_health: u32, damager_entity_id: ProcessUniqueId, damaged_entity_id: ProcessUniqueId, events: &mut EventMap) -> u32 {
        if self.invulnerable_timer == 0f64 {
            println!("{:?} {:?}", new_health, self.health);
            events.add(event::Event::new(DamageEvent {
                entity_id: damager_entity_id,
                amount: new_health as i32 - self.health as i32,
            }, damaged_entity_id, None));
            if new_health < self.health {
                self.invulnerable_timer = self.invulnerable_default;
            }
            self.health = new_health;
        }
        self.health
    }

    fn update(&mut self, event: &Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        let args = event.get_event_data::<update_event::UpdateEvent>().unwrap().args;
        if self.invulnerable_timer > args.dt {
            self.invulnerable_timer -= args.dt;
        } else {
            self.invulnerable_timer = 0f64;
        }
        DestroyType::None
    }
}
