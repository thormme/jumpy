extern crate tiled;

use damageable::Damageable;
use sprite::Sprite;
use entity_states::EntityStates;
use collidable::Collidable;
use piston_window::*;
use app::{ButtonStates};
use std::collections::*;
use entity::Entity;
use player::Player;
use self::tiled::Map;
use snowflake::ProcessUniqueId;
use nalgebra::{Vector2, Point2};

#[derive(Debug)]
pub struct Enemy {
    id: ProcessUniqueId,
    body: Collidable,
    sprite: String,
    player_id: ProcessUniqueId,
    health: u32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, sprite: String, player_id: ProcessUniqueId) -> Enemy {
        Enemy {
            id: ProcessUniqueId::new(),
            body: Collidable::new(Point2::new(x, y), Vector2::new(0f32, 0f32), vec![
                Point2::new(0f32, 0f32), Point2::new(0f32, 32f32),
                Point2::new(32f32, 32f32), Point2::new(32f32, 0f32)
            ]),
            sprite: sprite,
            player_id: player_id,
            health: 1u32,
        }
    }
}

impl Damageable for Enemy {
    fn get_health(&self) -> u32 {
        self.health
    }
    fn set_health(&mut self, health: u32) -> u32 {
        self.health = health;
        self.health
    }
}

impl Entity for Enemy {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        if let Some(player_opt) = entities.get(&self.player_id) {
            if let Some(player) = player_opt.as_any().downcast_ref::<Player>() {
                if let &Some(body) = &player.get_body() {
                    let pos = body.pos;
                    let direction = (pos.y-self.body.pos.y).atan2(pos.x-self.body.pos.x);
                    self.body.pos.x += direction.cos()*10f32 * args.dt as f32;
                    self.body.pos.y += direction.sin()*10f32 * args.dt as f32;
                }
            }
        }
        if self.health == 0u32 {
            return true;
        }

        false
    }
    fn draw(&mut self, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
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
            &sprites.get(&self.sprite).unwrap().texture,
            &DrawState::default(),
            trans,
            gl,
        );
    }

    fn get_body(&self) -> Option<&Collidable> {
        Some(&self.body)
    }

    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }

    fn get_damageable(&mut self) -> Option<&mut Damageable> {
        Some(self)
    }
}
