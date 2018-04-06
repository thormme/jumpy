extern crate tiled;

use component::DestroyType;
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
use sprite::AnimationState;
use component_states::ComponentHashMap;

#[derive(Debug)]
pub struct Enemy {
    player_id: ProcessUniqueId,
    health: u32,
}

impl Enemy {
    pub fn new(player_id: ProcessUniqueId) -> Enemy {
        Enemy {
            player_id: player_id,
            health: 1u32,
        }
    }

    pub fn new_entity(x: f32, y: f32, player_id: ProcessUniqueId) -> Entity {
        let mut components = ComponentStates::new();
        components.insert_component(Collidable::new(Point2::new(x, y), Vector2::new(0f32, 0f32), vec![
            Point2::new(0f32, 0f32), Point2::new(0f32, 32f32),
            Point2::new(32f32, 32f32), Point2::new(32f32, 0f32)
        ]));
        components.insert_component(AnimationState::new("enemy".to_string(), "stand".to_string(), None));
        components.insert_component(Enemy::new(player_id));
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
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> DestroyType {
        if let Some(player) = entities.get_mut(&self.player_id) {
            if player.components.get_component::<Player>().is_some() {
                if let Some(other_body) = player.components.get_mut_component::<Collidable>() {
                    if let Some(body) = entity.components.get_mut_component::<Collidable>() {
                        let pos = other_body.pos;
                        let direction = (pos.y-body.pos.y).atan2(pos.x-body.pos.x);
                        body.pos.x += direction.cos()*10f32 * args.dt as f32;
                        body.pos.y += direction.sin()*10f32 * args.dt as f32;
                    }
                }
            }
        }
        if self.health == 0u32 {
            return DestroyType::Entity;
        }

        DestroyType::None
    }
}
