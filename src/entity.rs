extern crate tiled;

use sprite::Sprite;
use entity_states::EntityStates;
use collidable::Collidable;
use damageable::Damageable;
use piston_window::*;
use app::{ButtonStates};
use std::any::Any;
use std::collections::*;
use std::cmp::Eq;
use self::tiled::Map;
use snowflake::ProcessUniqueId;

pub trait Entity : AsAny {
    fn update(&mut self, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool;
    fn draw(&mut self, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprite: &HashMap<String, Sprite>);
    fn get_body(&self) -> Option<&Collidable>;
    fn get_damageable(&mut self) -> Option<&mut Damageable>;
    fn get_id(&self) -> ProcessUniqueId;
}

impl PartialEq for Entity {
  fn eq(&self, other: &Entity) -> bool {
    self.get_id() == other.get_id()
  }
}

impl Eq for Entity {}

pub trait AsAny {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}
