extern crate tiled;
extern crate find_folder;
extern crate graphics;
extern crate std;
extern crate snowflake;
extern crate evmap;
use entity_states::EntityStates;
use sdl2_window::Sdl2Window;
use std::fs::File;
use std::vec::Vec;
use self::tiled::parse;
use piston_window::*;
//use piston::event_loop::*;
//use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL };
use entity::Entity;
use player::{ Player };
use enemy::Enemy;
use ball::Ball;
use std::collections::HashMap;
use snowflake::ProcessUniqueId;

/*pub trait CollidableGrid {
    fn get_
}*/

pub trait ButtonStates {
    fn get_button_down(&self, button: &Button) -> bool;
}

impl ButtonStates for HashMap<Button, bool> {
    fn get_button_down(&self, button: &Button) -> bool {
        if let Some(pressed) = self.get(button) {
            pressed.clone()
        } else {
            false
        }
    }
}

pub struct App {
    pub window: PistonWindow<Sdl2Window>, // OpenGL drawing backend.
    map: self::tiled::Map,
    tilesheet: G2dTexture,
    image: Image,
    entities: EntityStates,
    keys: HashMap<Button, bool>,
    sprites: HashMap<String, G2dTexture>
}

impl App {
    pub fn new() -> App {

            let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap();

            let file = File::open(assets.join("tiled_base64_zlib.tmx")).unwrap();
            let map = parse(file).unwrap();

            // Change this to OpenGL::V2_1 if not working.
            let opengl = OpenGL::V3_2;

            // Create an Glutin window.
            let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(
                    "spinning-square",
                    [640, 480]
                )
                .opengl(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();


            let tileset = map.get_tileset_by_gid(1).unwrap().clone();


            let tilesheet = assets.join(&tileset.images[0].source);
            let tilesheet = Texture::from_path(
                &mut window.factory,
                &tilesheet,
                Flip::None,
                &TextureSettings::new(),
            ).unwrap();
            let image = Image::new();

            // Create a new game and run it.
            let mut app = App {
                window: window,
                map: map,
                tilesheet: tilesheet,
                image: image,
                entities: EntityStates::new(),
                keys: HashMap::new(),
                sprites: HashMap::new()
            };

            let player_sprite_path = assets.join("player.png");
            let player_sprite = Texture::from_path(
                &mut app.window.factory,
                &player_sprite_path,
                Flip::None,
                &TextureSettings::new(),
            ).unwrap();
            let enemy_sprite_path = assets.join("enemy.png");
            let enemy_sprite = Texture::from_path(
                &mut app.window.factory,
                &enemy_sprite_path,
                Flip::None,
                &TextureSettings::new(),
            ).unwrap();
            app.sprites.insert("player".to_owned(), player_sprite);
            app.sprites.insert("enemy".to_owned(), enemy_sprite);
            let player = Player::new(33.0, 5.0, "player".to_owned());
            let enemy = Enemy::new(50.0, 50.0, "enemy".to_owned(), player.get_id());
            app.entities.insert(player.get_id(), Box::new(player));
            app.entities.insert(enemy.get_id(), Box::new(enemy));
            let ball = Ball::new(50.0, 100.0, 2.0, 0.0, "enemy".to_owned());
            app.entities.insert(ball.get_id(), Box::new(ball));

            app
    }

    pub fn render(&mut self, event: Event, args: &RenderArgs) {

        let tileset = self.map.get_tileset_by_gid(1).unwrap();
        let tile_width = tileset.tile_width;
        let tile_height = tileset.tile_height;

        let (width, _) = self.tilesheet.get_size();
        let layer: &tiled::Layer = &self.map.layers[0];

        let image = &self.image;
        let tilesheet = &self.tilesheet;
        let entities = &self.entities;
        let sprites = &self.sprites;
        self.window.draw_2d(&event, |context, gl| {
            clear([1.0; 4], gl);

            for (y, row) in layer.tiles.iter().enumerate() {
                for (x, &tile) in row.iter().enumerate() {
                    if tile == 0 {
                        continue;
                    }

                    let tile = tile - 1; // tiled counts from 1

                    // rect of the particular tile in the tilesheet
                    let src_rect = [
                        (tile % (width / tile_width) * tile_width) as f64,
                        (tile / (width / tile_height) * tile_height) as f64,
                        tile_width as f64,
                        tile_height as f64,
                    ];

                    let trans = context.transform.trans(
                        x as f64 * tile_width as f64,
                        y as f64 * tile_height as f64,
                    );

                    image.src_rect(src_rect).draw(
                        tilesheet,
                        &DrawState::default(),
                        trans,
                        gl,
                    );
                }
            }

            entities.for_each(|entity| {
                entity.draw(&event, args, &image, &context, gl, &sprites);
            });
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let map = &self.map;
        let keys = &self.keys;
        let entity_ids: Vec<ProcessUniqueId> = self.entities.keys().cloned().collect();
        for id in entity_ids {
            let mut entity_result = self.entities.remove(&id);
            if let Some(mut entity) = entity_result {
                entity.update(args, keys, &mut self.entities, &map);
                self.entities.insert(id, entity);
            }
        }
    }

    pub fn handle_press(&mut self, button: Button) {
        self.keys.insert(button, true);
    }

    pub fn handle_release(&mut self, button: Button) {
        self.keys.insert(button, false);
    }
}
