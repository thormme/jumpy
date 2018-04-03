extern crate tiled;
extern crate nalgebra;

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

#[derive(Debug)]
pub struct Bullet {
    animation: AnimationState,
}

impl Bullet {
    pub fn new() -> Self {
        Bullet {
            animation: AnimationState::new("ball".to_string(), "still".to_string()),
        }
    }

    pub fn new_entity(x: f32, y: f32, dx: f32, dy: f32) -> Entity {
        let mut components = ComponentStates::new();
        components.insert(Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
            Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
            Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
        ]));
        components.insert(Bullet::new());
        Entity::new(components)
    }
}

impl Component for Bullet {
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        let mut destroy = false;
        if let Some(body) = entity.components.get_mut::<Collidable>() {

            entities.for_zone(body.pos, 1, |colliding_entity| {
                let mut colliding = false;
                if let Some(other_body) = colliding_entity.components.get::<Collidable>() {
                    if other_body.is_colliding(&body) {
                        colliding = true;
                    }
                }
                if colliding {
                    //if let Some(damageable) = colliding_entity.get_damageable() {
                    if let Some(enemy) = colliding_entity.components.get_mut::<Enemy>() {
                        enemy.set_health(0);
                        destroy = true;
                    }
                }
            });
        }

        destroy
    }

    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        if let Some(body) = entity.components.get::<Collidable>() {
            let pos = &body.pos;
            self.animation.draw(args, image, context, gl, sprites, |_src_rect| {
                context.transform.trans(
                    pos.x as f64,
                    pos.y as f64,
                )
            });
        }
    }
}
