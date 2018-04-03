
use std::fmt::Debug;
use entity::AsAny;
use sprite::Sprite;
use std::collections::HashMap;
use piston_window::G2d;
use piston_window::Context;
use piston_window::Image;
use piston_window::RenderArgs;
use piston_window::Event;
use entity::Entity;
use tiled::Map;
use entity_states::EntityStates;
use app::ButtonStates;
use piston::input::UpdateArgs;

pub trait Component : AsAny + Debug {
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> bool {false}
    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {}
}
