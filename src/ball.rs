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
use collidable::Collidable;
use player::Player;
use self::tiled::{Map, PropertyValue, Tile};
use self::nalgebra::{Vector2, Point2, Similarity2};
use snowflake::ProcessUniqueId;

#[derive(Debug)]
pub struct Ball {
    id: ProcessUniqueId,
    body: Collidable,
    sprite: String,
}

impl Ball {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32, sprite: String) -> Ball {
        Ball {
            id: ProcessUniqueId::new(),
            body: Collidable {
                pos: Point2::new(x, y),
                speed: Vector2::new(dx, dy),
                grounded: false,
            },
            sprite: sprite,
        }
    }
}

impl Entity for Ball {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut HashMap<ProcessUniqueId, Box<Entity>>, map: &Map) {
        self.body.speed.y += 0.1f32;
        let prev_pos = self.body.pos.clone();
        self.body.pos += self.body.speed;

        let points = vec![Point2::new(5f32, 1f32), Point2::new(5f32, 32f32),
            Point2::new(27f32, 32f32), Point2::new(27f32, 1f32)];
        let prev_speed = self.body.speed;
        self.body.handle_collisions(&points, map, &prev_pos);
        self.body.speed -= prev_speed - self.body.speed;
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
