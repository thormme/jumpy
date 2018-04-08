
use app::EventMap;
use snowflake::ProcessUniqueId;
use event::EventData;
use std::fmt;
use std::any::TypeId;
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
use event;

pub enum DestroyType { Component, Entity, None}

pub trait Component : AsAny + Debug {
    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {}
    fn handle_event(&mut self, event_type: TypeId, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) ->  DestroyType {
        DestroyType::None
    }
}
