extern crate tiled;

use component::Component;
use component_states::ComponentStates;
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
    sprite: String,
    player_id: ProcessUniqueId,
    health: u32,
}

impl Enemy {
    pub fn new(sprite: String, player_id: ProcessUniqueId) -> Enemy {
        Enemy {
            sprite: sprite,
            player_id: player_id,
            health: 1u32,
        }
    }

    pub fn new_entity(x: f32, y: f32, sprite: String, player_id: ProcessUniqueId) -> Entity {
        let mut components = ComponentStates::new();
        components.insert(Collidable::new(Point2::new(x, y), Vector2::new(0f32, 0f32), vec![
            Point2::new(0f32, 0f32), Point2::new(0f32, 32f32),
            Point2::new(32f32, 32f32), Point2::new(32f32, 0f32)
        ]));
        components.insert(Enemy::new(sprite, player_id));
        Entity::new(components)
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

impl Component for Enemy {
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        if let Some(player) = entities.get_mut(&self.player_id) {
            if player.components.get::<Player>().is_some() {
                if let Some(other_body) = player.components.get_mut::<Collidable>() {
                    if let Some(body) = entity.components.get_mut::<Collidable>() {
                        let pos = other_body.pos;
                        let direction = (pos.y-body.pos.y).atan2(pos.x-body.pos.x);
                        body.pos.x += direction.cos()*10f32 * args.dt as f32;
                        body.pos.y += direction.sin()*10f32 * args.dt as f32;
                    }
                }
            }
        }
        if self.health == 0u32 {
            return true;
        }

        false
    }
    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        if let Some(body) = entity.components.get::<Collidable>() {
            let src_rect = [
                0f64,
                0f64,
                32f64,
                32f64,
            ];

            let trans = context.transform.trans(
                body.pos.x as f64,
                body.pos.y as f64,
            );

            image.src_rect(src_rect).draw(
                &sprites.get(&self.sprite).unwrap().texture,
                &DrawState::default(),
                trans,
                gl,
            );
        }
    }
}
