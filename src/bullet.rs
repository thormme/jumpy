extern crate tiled;
extern crate nalgebra;

use app::EventMap;
use std::any::TypeId;
use component::DestroyType;
use enemy::Enemy;
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
use component_states::ComponentHashMap;
use update_event;
use event;

#[derive(Debug)]
pub struct Bullet {
    animation: AnimationState,
}

impl Bullet {
    pub fn new() -> Self {
        Bullet {
            animation: AnimationState::new("ball".to_string(), "still".to_string(), None),
        }
    }

    pub fn new_entity(x: f32, y: f32, dx: f32, dy: f32) -> Entity {
        let mut components = ComponentStates::new();
        components.insert_component(Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
            Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
            Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
        ]));
        components.insert_component(Bullet::new());
        Entity::new(components)
    }

    fn update(&mut self, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        let mut destroy = DestroyType::None;
        let entity_id = entity.get_id();
        if let Some(body) = entity.components.get_mut_component::<Collidable>() {
            entities.for_zone(body.pos, 1, |colliding_entity| {
                let mut colliding = false;
                if let Some(other_body) = colliding_entity.components.get_component::<Collidable>() {
                    if other_body.is_colliding(&body) {
                        colliding = true;
                    }
                }
                if colliding {
                    let colliding_id = colliding_entity.get_id();
                    if let Some(damageable) = colliding_entity.components.get_mut_component::<Damageable>() {
                        damageable.set_health(0, entity_id, colliding_id, events);
                        destroy = DestroyType::Entity;
                    }
                }
            });
        }

        destroy
    }
}

impl Component for Bullet {
    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        if let Some(body) = entity.components.get_component::<Collidable>() {
            let pos = &body.pos;
            self.animation.draw(args, image, context, gl, sprites, |_src_rect| {
                context.transform.trans(
                    pos.x as f64,
                    pos.y as f64,
                )
            });
        }
    }

    fn handle_event(&mut self, event_type: TypeId, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        if event_type == TypeId::of::<update_event::UpdateEvent>() {
            self.update(event, entity, keys, entities, map, events)
        } else {
            DestroyType::None
        }
    }
}
