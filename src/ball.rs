extern crate tiled;
extern crate nalgebra;

use app::EventMap;
use component::DestroyType;
use component::Component;
use component_states::ComponentStates;
use damageable::Damageable;
use sprite::AnimationState;
use sprite::Sprite;
use entity_states::EntityStates;
use piston_window::*;
use app::{ButtonStates};
use std::collections::*;
use entity::Entity;
use collidable::Collidable;
use player::Player;
use self::tiled::{Map};
use self::nalgebra::{Vector2, Point2};
use snowflake::ProcessUniqueId;
use component_states::{ ComponentHashMap, MapByTypeId };
use splitmut::SplitMut;
use std::any::TypeId;
use event;
use update_event;

#[derive(Debug)]
pub struct Ball {
    prev_speed: Vector2<f32>,
}

impl Ball {
    pub fn new(dx: f32, dy: f32) -> Ball {
        Ball {
            prev_speed: Vector2::new(dx, dy),
        }
    }

    pub fn new_entity(x: f32, y: f32, dx: f32, dy: f32) -> Entity {
        let mut components = ComponentStates::new();
        components.insert_component(Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
            Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
            Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
        ]));
        components.insert_component(AnimationState::new("ball".to_string(), "still".to_string(), None));
        components.insert_component(Ball::new(dx, dy));
        Entity::new(components)
    }

    fn update(&mut self, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        let mut components = entity.components.get_muts();
        if let Some(body) = components.get_mut::<Collidable>() {
            body.speed -= self.prev_speed - body.speed;

            body.speed.y += 0.1f32;
            self.prev_speed = body.speed;

            if body.grounded {
                if let Some(animation) = components.get_mut::<AnimationState>() {
                    animation.set_animation("bounce".to_owned());
                }
            }

            entities.for_zone(body.pos, 1, |entity| {
                if entity.components.get_component::<Player>().is_some() {
                    /*if let Some(body) = components.get_mut::<Collidable>() {
                        if body.is_colliding(&body) {
                            //self.animation.set_sprite("player".to_owned(), "run".to_owned());
                        }
                    }*/
                }
            });
        }

        DestroyType::None
    }
}

impl Component for Ball {
    fn handle_event(&mut self, event_type: TypeId, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        if event_type == TypeId::of::<update_event::UpdateEvent>() {
            self.update(event, entity, keys, entities, map, events)
        } else {
            DestroyType::None
        }
    }
}
