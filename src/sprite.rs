
use component::DestroyType;
use app::EventMap;
use entity_states::EntityStates;
use app::ButtonStates;
use std::any::TypeId;
use collidable::Collidable;
use std::*;
use player::Player;
use std::fmt::Debug;
use entity::Entity;
use component::Component;
use piston_window::G2d;
use piston_window::*;
use piston::input::RenderArgs;
use sdl2_window::Sdl2Window;
use piston_window::{G2dTexture, PistonWindow, Texture, Flip, TextureSettings};
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use serde_json;
use std::fs::File;
use component_states::ComponentHashMap;
use update_event;
use event;
use tiled::Map;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationFrame {
    pub name: String,
    pub length: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub next: String,
    pub frames: Vec<AnimationFrame>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FrameData {
    pub frame: Frame,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub image: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteData {
    pub animations: HashMap<String, Animation>,
    pub frames: HashMap<String, FrameData>,
    pub meta: Metadata,
}

pub struct Sprite {
    pub sprite_data: SpriteData,
    pub texture: G2dTexture,
}

impl Sprite {
    pub fn new(file_path: &Path, window: &mut PistonWindow<Sdl2Window>) -> Result<Sprite, Box<Error>> {
        println!("{:?}", file_path);
        let file = File::open(file_path)?;
        println!("open");
        let sprite_data: SpriteData = serde_json::from_reader(file)?;
        println!("parsed");
        let texture_path = file_path.with_file_name(&sprite_data.meta.image);
        println!("{:?}", texture_path);
        let texture = Texture::from_path(
            &mut window.factory,
            &texture_path,
            Flip::None,
            &TextureSettings::new(),
        )?;
        Ok(Sprite {
            sprite_data: sprite_data,
            texture: texture,
        })
    }
}

impl Frame {
    pub fn as_rect(&self) -> [f64; 4] {
        [
            self.x as f64,
            self.y as f64,
            self.w as f64,
            self.h as f64,
        ]
    }
}

pub struct AnimationState {
    pub sprite_name: String,
    pub animation_name: String,
    pub frame: usize,
    frame_time: f64,
    transform_generator: fn(&Entity, &Event, &RenderArgs, &Context, &[f64; 4]) -> [[f64; 3]; 2],
}

impl Debug for AnimationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AnimationState {{ sprite_name: {}, animation_name: {}, frame: {}, frame_time: {} }}",
            self.sprite_name, self.animation_name, self.frame, self.frame_time)
    }
}

type TranforGenerator = fn(&Entity, &Event, &RenderArgs, &Context, &[f64; 4]) -> [[f64; 3]; 2];

impl AnimationState {
    pub fn new(sprite_name: String, animation_name: String, transform_generator: Option<TranforGenerator>) -> Self {
        AnimationState {
            sprite_name: sprite_name,
            animation_name: animation_name,
            frame: 0usize,
            frame_time: 0f64,
            transform_generator: transform_generator.unwrap_or(Self::get_draw_transform),
        }
    }

    fn get_draw_transform(entity: &Entity, event: &Event, args: &RenderArgs, context: &Context, src_rect: &[f64; 4]) -> [[f64; 3]; 2] {
        if let Some(body) = entity.components.get_component::<Collidable>() {
            context.transform.trans(body.pos.x as f64, body.pos.y as f64)
        } else {
            context.transform
        }
    }

    pub fn advance(&mut self, dt: f64, sprites: &HashMap<String, Sprite>) {
        self.frame_time += dt;
        let animation = sprites.get(&self.sprite_name).unwrap().sprite_data.animations.get(&self.animation_name).unwrap();
        if self.frame_time >= animation.frames[self.frame].length as f64 {
            self.frame += 1;
            if self.frame >= animation.frames.len() {
                self.set_animation(animation.next.clone());
                self.frame = 0;
            }
            self.frame_time = 0f64;
        }
    }

    pub fn set_animation(&mut self, name: String) {
        if self.animation_name != name {
            self.frame = 0;
            self.animation_name = name;
            self.frame_time = 0f64;
        }
    }

    pub fn set_sprite(&mut self, sprite_name: String, animation_name: String) {
        if self.sprite_name != sprite_name && self.animation_name != animation_name {
            self.sprite_name = sprite_name;
            self.frame = 0;
            self.animation_name = animation_name;
            self.frame_time = 0f64;
        }
    }

    pub fn draw<F>(&mut self, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>, trans: F) where F : FnOnce(&[f64; 4]) -> [[f64; 3]; 2] {
        self.advance(1f64/60f64, sprites);
        let sprite = sprites.get(&self.sprite_name).unwrap();
        let animation = &sprite.sprite_data.animations.get(&self.animation_name).unwrap();
        let src_rect = sprite.sprite_data.frames.get(&animation.frames[self.frame].name).unwrap().frame.as_rect();

        image.src_rect(src_rect).draw(
            &sprite.texture,
            &DrawState::default(),
            trans(&src_rect),
            gl,
        );
    }
}

impl Component for AnimationState {
    fn draw(&mut self, entity: &mut Entity, event: &Event, args: &RenderArgs, image: &Image, context: &Context, gl: &mut G2d, sprites: &HashMap<String, Sprite>) {
        self.advance(1f64/60f64, sprites);
        let sprite = sprites.get(&self.sprite_name).unwrap();
        let animation = &sprite.sprite_data.animations.get(&self.animation_name).unwrap();
        let src_rect = sprite.sprite_data.frames.get(&animation.frames[self.frame].name).unwrap().frame.as_rect();

        image.src_rect(src_rect).draw(
            &sprite.texture,
            &DrawState::default(),
            (self.transform_generator)(entity, event, args, context, &src_rect),
            gl,
        );
    }

    fn handle_event(&mut self, event_type: TypeId, event: &event::Event, entity: &mut Entity, keys: &ButtonStates, entities: &mut EntityStates, map: &Map, events: &mut EventMap) -> DestroyType {
        DestroyType::None
    }
}
