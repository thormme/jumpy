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
pub struct Player {
    id: ProcessUniqueId,
    body: Collidable,
    jumping: bool,
    sprite: String,
}

impl Player {
    pub fn new(x: f32, y: f32, sprite: String) -> Player {
        Player {
            id: ProcessUniqueId::new(),
            body: Collidable {
                pos: Point2::new(x, y),
                speed: Vector2::new(0f32, 0f32),
                grounded: false,
            },
            jumping: false,
            sprite: sprite,
        }
    }
}

impl Entity for Player {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut HashMap<ProcessUniqueId, Box<Entity>>, map: &Map) {
        self.body.speed.y += 0.1f32;
        let prev_pos = self.body.pos.clone();
        if keys.get_button_down(&Button::Keyboard(Key::Right)) {
            self.body.pos.x += 1f32;
        }
        if keys.get_button_down(&Button::Keyboard(Key::Left)) {
            self.body.pos.x -= 1f32;
        }
        if keys.get_button_down(&Button::Keyboard(Key::Up)) {
            if self.body.grounded {
                self.body.speed.y = -4f32;
                self.jumping = true;
            }
        } else if self.jumping {
            self.jumping = false;
            if self.body.speed.y < -1f32 {
                self.body.speed.y = -1f32;
            }
        }
        if keys.get_button_down(&Button::Keyboard(Key::Down)) {
            self.body.pos.y += 1f32;
        }
        if keys.get_button_down(&Button::Keyboard(Key::X)) {
            let ball = Ball::new(self.body.pos.x, self.body.pos.y, 2f32, -1f32, self.sprite.clone());
            entities.insert(ball.get_id(), Box::new(ball));
        }
        self.body.pos += self.body.speed;

        let points = vec![Point2::new(5f32, 1f32), Point2::new(5f32, 32f32),
            Point2::new(27f32, 32f32), Point2::new(27f32, 1f32)];
        self.body.handle_collisions(&points, map, &prev_pos);
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
            32f64,
            32f64,
        ];

        let trans = context.transform.trans(
            self.body.pos.x as f64,
            self.body.pos.y as f64,
        );

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
