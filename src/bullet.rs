extern crate tiled;
extern crate nalgebra;

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
    id: ProcessUniqueId,
    body: Collidable,
    animation: AnimationState,
}

impl Bullet {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Bullet {
            id: ProcessUniqueId::new(),
            body: Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
                Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
                Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
            ]),
            animation: AnimationState::new("ball".to_string(), "still".to_string()),
        }
    }
}

impl Entity for Bullet {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        let mut destroy = false;
        let prev_pos = self.body.pos.clone();
        self.body.pos += self.body.speed;

        //self.body.handle_collisions(map, &prev_pos);

        entities.for_zone(self.body.pos, 1, |entity| {
            let mut colliding = false;
            if let &Some(body) = &entity.get_body() {
                if body.is_colliding(&self.body) {
                    colliding = true;
                }
            }
            if colliding {
                if let Some(damageable) = entity.get_damageable() {
                    damageable.set_health(0);
                    destroy = true;
                }
            }
        });

        destroy
    }
    fn draw(&mut self, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        let pos = &self.body.pos;
        self.animation.draw(args, image, context, gl, sprites, |_src_rect| {
            context.transform.trans(
                pos.x as f64,
                pos.y as f64,
            )
        });
    }

    fn get_body(&self) -> Option<&Collidable> {
        return Some(&self.body);
    }

    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }
}