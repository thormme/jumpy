extern crate piston;
extern crate opengl_graphics;
extern crate piston_window;
extern crate snowflake;
extern crate evmap;
extern crate nalgebra;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use piston_window::*;
use app::App;

mod app;
mod player;
mod enemy;
mod entity;
mod collidable;
mod ball;
mod entity_states;
mod sprite;

extern crate sdl2_window;

fn main() {
    let mut app = App::new();

    let mut event_settings = EventSettings::new();
    event_settings.ups = 60;
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut app.window) {
        let render_some = e.render_args();
        let update_some = e.update_args();
        let press_some = e.press_args();
        let release_some = e.release_args();

        if let Some(r) = render_some {
            app.render(e, &r);
        }

        if let Some(p) = press_some {
            app.handle_press(p);
        }

        if let Some(r) = release_some {
            app.handle_release(r);
        }

        if let Some(u) = update_some {
            app.update(&u);
        }
    }
}
