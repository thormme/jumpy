extern crate tiled;
extern crate nalgebra;

use std::any::TypeId;
use component_states::ComponentStates;
use damageable::Damageable;
use sprite::AnimationState;
use sprite::Sprite;
use entity_states::EntityStates;
use piston_window::*;
use piston::input::Key;
use app::{ButtonStates};
use std::collections::*;
use entity::Entity;
use bullet::Bullet;
use collidable::Collidable;
use component::Component;
use self::tiled::{Map};
use self::nalgebra::{Vector2, Point2};
use snowflake::ProcessUniqueId;

#[derive(Debug)]
enum FacingDirection {Left, Right}

#[derive(Debug)]
pub struct Player {
    jumping: bool,
    facing: FacingDirection,
    animation: AnimationState,
}

impl Player {
    pub fn new(animation: AnimationState) -> Player {
        Player {
            jumping: false,
            facing: FacingDirection::Right,
            animation: animation,
        }
    }

    pub fn new_entity(x: f32, y: f32, animation: AnimationState) -> Entity {
        let mut components = ComponentStates::new();
        components.insert(Collidable::new(Point2::new(x, y), Vector2::new(0f32, 0f32), vec![
            Point2::new(0f32, 0f32), Point2::new(0f32, 48f32),
            Point2::new(31f32, 24f32), Point2::new(0f32, 24f32),
            Point2::new(31f32, 48f32), Point2::new(31f32, 0f32)
        ]));
        components.insert(Player::new(animation));
        Entity::new(components)
    }
}

impl Component for Player {
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        if let Some(body) = entity.components.get_mut::<Collidable>() {
            body.speed.y += 0.5f32;
            body.speed.x *= 0.8f32;
            if keys.get_button_down(&Button::Keyboard(Key::Right)) {
                self.animation.set_animation("run".to_owned());
                body.speed.x += 1f32;
                self.facing = FacingDirection::Right;
            } else if keys.get_button_down(&Button::Keyboard(Key::Left)) {
                self.animation.set_animation("run".to_owned());
                body.speed.x -= 1f32;
                self.facing = FacingDirection::Left;
            } else {
                self.animation.set_animation("stand".to_owned());
            }
            if keys.get_button_down(&Button::Keyboard(Key::Up)) {
                if body.grounded {
                    body.speed.y = -10f32;
                    self.jumping = true;
                }
            } else if self.jumping {
                self.jumping = false;
                if body.speed.y < -2.5f32 {
                    body.speed.y = -2.5f32;
                }
            }
            if keys.get_button_down(&Button::Keyboard(Key::Down)) {
                body.pos.y += 1f32;
            }
            if keys.get_button_down(&Button::Keyboard(Key::X)) {
                let x_speed = match self.facing { FacingDirection::Left => -8f32, FacingDirection::Right => 8f32 };
                let bullet = Bullet::new_entity(body.pos.x, body.pos.y, x_speed, 0f32);
                entities.insert(bullet.get_id(), Box::new(bullet));
            }
            if body.speed.x > 3f32 {
                body.speed.x = 3f32;
            }
            if body.speed.x < -3f32 {
                body.speed.x = -3f32;
            }
        }

        entities.for_each(|entity| {
            if let Some(_player) = entity.components.get::<Player>() {
                println!("{:?}", entity);
            }
        });

        false
    }

    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        if let Some(body) = entity.components.get::<Collidable>() {
            let pos = &body.pos;
            let facing = &self.facing;
            self.animation.draw(args, image, context, gl, sprites, |src_rect| {
                match facing {
                    &FacingDirection::Right => context.transform.trans(
                            pos.x as f64 - 2f64,
                            pos.y as f64,
                        ),
                    &FacingDirection::Left => context.transform.trans(
                            pos.x as f64 + src_rect[2] - 1f64,
                            pos.y as f64,
                        ).flip_h(),
                }
            });
        }
    }
}
