extern crate tiled;
extern crate nalgebra;

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
    id: ProcessUniqueId,
    body: Collidable,
    animation: AnimationState,
    components: ComponentStates,
}

impl Ball {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Ball {
        let mut components = ComponentStates::new();
        components.insert(Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
            Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
            Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
        ]));
        Ball {
            id: ProcessUniqueId::new(),
            body: Collidable::new(Point2::new(x, y), Vector2::new(dx, dy), vec![
                Point2::new(0f32, 1f32), Point2::new(0f32, 16f32),
                Point2::new(15f32, 16f32), Point2::new(15f32, 1f32)
            ]),
            animation: AnimationState::new("ball".to_string(), "still".to_string()),
            components: components,
        }
    }
}

impl Entity for Ball {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {
        self.body.speed.y += 0.1f32;
        let prev_pos = self.body.pos.clone();
        self.body.pos += self.body.speed;

        let prev_speed = self.body.speed;
        self.body.handle_collisions(map, &prev_pos);
        self.body.speed -= prev_speed - self.body.speed;

        if self.body.grounded {
            self.animation.set_animation("bounce".to_owned());
        }

        entities.for_zone(self.body.pos, 1, |entity| {
            if let Some(player) = entity.as_any().downcast_ref::<Player>() {
                if let &Some(body) = &player.get_body() {
                    if body.is_colliding(&self.body) {
                        //self.animation.set_sprite("player".to_owned(), "run".to_owned());
                    }
                }
            }
        });

        false
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

    fn get_components(&self) -> &ComponentStates {
        &self.components
    }

    fn get_components_mut(&mut self) -> &mut ComponentStates {
        &mut self.components
    }
}
