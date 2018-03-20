extern crate tiled;
extern crate nalgebra;

use piston_window::*;
use piston::input::Key;
use app::{ButtonStates};
use std::iter::*;
use std::slice::*;
use std::any::Any;
use std::collections::*;
use entity::Entity;
use ball::Ball;
use collidable::Collidable;
use self::tiled::{Map, PropertyValue, Tile};
use self::nalgebra::{Vector2, Point2, Similarity2};
use snowflake::ProcessUniqueId;

#[derive(Debug)]
enum FacingDirection {Left, Right}

#[derive(Debug)]
pub struct Player {
    id: ProcessUniqueId,
    body: Collidable,
    jumping: bool,
    facing: FacingDirection,
    sprite: String,
}

impl Player {
    pub fn new(x: f32, y: f32, sprite: String) -> Player {
        Player {
            id: ProcessUniqueId::new(),
            body: Collidable::new(Point2::new(x, y), Vector2::new(0f32, 0f32), vec![
                Point2::new(0f32, 0f32), Point2::new(0f32, 48f32),
                Point2::new(31f32, 24f32), Point2::new(0f32, 24f32),
                Point2::new(31f32, 48f32), Point2::new(31f32, 0f32)
            ]),
            jumping: false,
            facing: FacingDirection::Right,
            sprite: sprite,
        }
    }
}

impl Entity for Player {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut HashMap<ProcessUniqueId, Box<Entity>>, map: &Map) {
        self.body.speed.y += 0.5f32;
        let prev_pos = self.body.pos.clone();
        self.body.speed.x *= 0.8f32;
        if keys.get_button_down(&Button::Keyboard(Key::Right)) {
            self.body.speed.x += 1f32;
            self.facing = FacingDirection::Right;
        }
        if keys.get_button_down(&Button::Keyboard(Key::Left)) {
            self.body.speed.x -= 1f32;
            self.facing = FacingDirection::Left;
        }
        if keys.get_button_down(&Button::Keyboard(Key::Up)) {
            if self.body.grounded {
                self.body.speed.y = -10f32;
                self.jumping = true;
            }
        } else if self.jumping {
            self.jumping = false;
            if self.body.speed.y < -2.5f32 {
                self.body.speed.y = -2.5f32;
            }
        }
        if keys.get_button_down(&Button::Keyboard(Key::Down)) {
            self.body.pos.y += 1f32;
        }
        if keys.get_button_down(&Button::Keyboard(Key::X)) {
            let x_speed = match self.facing { FacingDirection::Left => -2f32, FacingDirection::Right => 2f32 };
            let ball = Ball::new(self.body.pos.x, self.body.pos.y, x_speed, -1f32, self.sprite.clone());
            entities.insert(ball.get_id(), Box::new(ball));
        }
        if self.body.speed.x > 3f32 {
            self.body.speed.x = 3f32;
        }
        if self.body.speed.x < -3f32 {
            self.body.speed.x = -3f32;
        }
        self.body.pos += self.body.speed;

        self.body.handle_collisions(map, &prev_pos);
        //println!("{:?}", self.pos);

        for (_, entity) in entities {
            if let Some(player) = entity.as_any().downcast_ref::<Player>() {
                println!("{:?}", player);
            }
        }
    }

    fn draw(&self, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, G2dTexture>) {
        let src_rect = [
            0f64,
            0f64,
            34f64,
            49f64,
        ];

        let trans = match &self.facing {
            &FacingDirection::Right => context.transform.trans(
                    self.body.pos.x as f64 - 2f64,
                    self.body.pos.y as f64,
                ),
            &FacingDirection::Left => context.transform.trans(
                    self.body.pos.x as f64 + 33f64,
                    self.body.pos.y as f64,
                ).flip_h(),
        };

        image.src_rect(src_rect).draw(
            sprites.get(&self.sprite).unwrap(),
            &DrawState::default(),
            trans,
            gl,
        );
    }

    fn get_position(&self) -> (f32, f32) {
        return (self.body.pos.x, self.body.pos.y);
    }

    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }
}
