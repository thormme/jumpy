extern crate tiled;
extern crate nalgebra;

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
pub struct Ball {
    animation: AnimationState,
    prev_speed: Vector2<f32>,
}

impl Ball {
    pub fn new(dx: f32, dy: f32) -> Ball {
        Ball {
            animation: AnimationState::new("ball".to_string(), "still".to_string()),
            prev_speed: Vector2::new(dx, dy),
        }
    }

    pub fn new_entity(x: f32, y: f32, dx: f32, dy: f32) -> Entity {
        let mut components = ComponentStates::new();
        components.insert(Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
            Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
            Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
        ]));
        components.insert(Ball::new(dx, dy));
        Entity::new(components)
    }
}

impl Component for Ball {
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        if let Some(body) = entity.components.get_mut::<Collidable>() {
            body.speed -= self.prev_speed - body.speed;

            body.speed.y += 0.1f32;
            self.prev_speed = body.speed;

            if body.grounded {
                self.animation.set_animation("bounce".to_owned());
            }

            entities.for_zone(body.pos, 1, |entity| {
                if entity.components.get::<Player>().is_some() {
                    if let Some(body) = entity.components.get_mut::<Collidable>() {
                        if body.is_colliding(&body) {
                            //self.animation.set_sprite("player".to_owned(), "run".to_owned());
                        }
                    }
                }
            });
        }

        false
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
