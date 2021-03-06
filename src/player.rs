extern crate tiled;
extern crate nalgebra;

use event::EventArgs;
use app::EventMap;
use component::DestroyType;
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
use component_states::ComponentHashMap;
use update_event;
use event;

#[derive(Debug, Clone)]
enum FacingDirection {Left, Right}

#[derive(Debug)]
pub struct Player {
    jumping: bool,
    facing: FacingDirection,
}

impl Player {
    pub fn new() -> Player {

        Player {
            jumping: false,
            facing: FacingDirection::Right,
        }
    }

    pub fn new_entity(x: f32, y: f32) -> Entity {
        let mut components = ComponentStates::new();
        components.insert_component(Collidable::new(Point2::new(x, y), Vector2::new(0f32, 0f32), vec![
            Point2::new(0f32, 0f32), Point2::new(0f32, 48f32),
            Point2::new(31f32, 24f32), Point2::new(0f32, 24f32),
            Point2::new(31f32, 48f32), Point2::new(31f32, 0f32)
        ]));
        components.insert_component(AnimationState::new("player".to_owned(), "run".to_owned(), Some(Self::get_draw_transform)));
        components.insert_component(Player::new());
        Entity::new(components)
    }

    fn get_draw_transform(entity: &Entity, event: &Event, args: &RenderArgs, context: &Context, src_rect: &[f64; 4]) -> [[f64; 3]; 2] {
        let facing = if let Some(player) = entity.components.get_component::<Player>() { player.facing.clone() } else { FacingDirection::Right };
        if let Some(body) = entity.components.get_component::<Collidable>() {
            let pos = &body.pos;
            return match facing {
                FacingDirection::Right => context.transform.trans(
                        pos.x as f64 - 2f64,
                        pos.y as f64,
                    ),
                FacingDirection::Left => context.transform.trans(
                        pos.x as f64 + src_rect[2] - 1f64,
                        pos.y as f64,
                    ).flip_h(),
            }
        } else {
            context.transform
        }
    }

    fn update(&mut self, event: &event::Event, entity: &mut Entity, args: &mut EventArgs) -> DestroyType {
        let mut animation = "stand".to_string();
        if let Some(body) = entity.components.get_mut_component::<Collidable>() {
            body.speed.y += 0.5f32;
            body.speed.x *= 0.8f32;
            if args.keys.get_button_down(&Button::Keyboard(Key::Right)) {
                animation = "run".to_string();
                body.speed.x += 1f32;
                self.facing = FacingDirection::Right;
            } else if args.keys.get_button_down(&Button::Keyboard(Key::Left)) {
                animation = "run".to_string();
                body.speed.x -= 1f32;
                self.facing = FacingDirection::Left;
            }
            if args.keys.get_button_down(&Button::Keyboard(Key::Up)) {
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
            if args.keys.get_button_down(&Button::Keyboard(Key::X)) {
                let x_speed = match self.facing { FacingDirection::Left => -8f32, FacingDirection::Right => 8f32 };
                let bullet = Bullet::new_entity(body.pos.x, body.pos.y, x_speed, 0f32);
                args.entities.insert(bullet.get_id(), Box::new(bullet));
            }
            if body.speed.x > 3f32 {
                body.speed.x = 3f32;
            }
            if body.speed.x < -3f32 {
                body.speed.x = -3f32;
            }
        }

        if let Some(animation_state) = entity.components.get_mut_component::<AnimationState>() {
            animation_state.set_animation(animation);
        }

        args.entities.for_each(|entity| {
            if let Some(_player) = entity.components.get_component::<Player>() {
                println!("{:?}", entity);
            }
        });

        DestroyType::None
    }
}

impl Component for Player {
    fn handle_event(&mut self, event_type: TypeId, event: &event::Event, entity: &mut Entity, args: &mut EventArgs) -> DestroyType {
        if event_type == TypeId::of::<update_event::UpdateEvent>() {
            self.update(event, entity, args)
        } else {
            DestroyType::None
        }
    }
}
