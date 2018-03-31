extern crate tiled;

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

#[derive(Debug)]
pub struct Enemy {
    id: ProcessUniqueId,
    x: f32,
    y: f32,
    sprite: String,
    player_id: ProcessUniqueId,
}

impl Enemy {
    pub fn new(x: f32, y: f32, sprite: String, player_id: ProcessUniqueId) -> Enemy {
        Enemy {
            id: ProcessUniqueId::new(),
            x: x,
            y: y,
            sprite: sprite,
            player_id: player_id,
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) {
        if let Some(player_opt) = entities.get(&self.player_id) {
            if let Some(player) = player_opt.as_any().downcast_ref::<Player>() {
                if let &Some(body) = &player.get_body() {
                    let pos = body.pos;
                    let direction = (pos.y-self.y).atan2(pos.x-self.x);
                    self.x += direction.cos()*10f32 * args.dt as f32;
                    self.y += direction.sin()*10f32 * args.dt as f32;
                }
            }
        }
    }
    fn draw(&mut self, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        let src_rect = [
            0f64,
            0f64,
            32f64,
            32f64,
        ];

        let trans = context.transform.trans(
            self.x as f64,
            self.y as f64,
        );

        image.src_rect(src_rect).draw(
            &sprites.get(&self.sprite).unwrap().texture,
            &DrawState::default(),
            trans,
            gl,
        );
    }

    fn get_body(&self) -> Option<&Collidable> {
        return None;
    }

    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }
}
