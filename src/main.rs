extern crate piston;
extern crate opengl_graphics;
extern crate piston_window;
extern crate snowflake;
extern crate evmap;
extern crate nalgebra;
extern crate tiled;
extern crate splitmut;

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
mod damageable;
mod bullet;
mod component;
mod component_states;

extern crate sdl2_window;

fn main() {
    let app = App::new();
    event_loop::run(handle_event, app);
}

fn handle_event(e: Event, app: &mut App) {
    let render_some = e.render_args();
    let update_some = e.update_args();
    let press_some = e.press_args();
    let release_some = e.release_args();

    if let Some(r) = render_some {
        //app.window.window.make_current();
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

#[cfg(not(target_os = "emscripten"))]
mod event_loop {
    use app::App;
    use piston_window::Event;
    use piston::event_loop::{EventSettings, Events};

    pub fn run(handler: fn(e: Event, app: &mut App),
                  mut app: App) {
        let mut event_settings = EventSettings::new();
        event_settings.ups = 60;
        let mut events = Events::new(event_settings);
        while let Some(e) = events.next(&mut app.window) {
            handler(e, &mut app);
        }
    }
}

#[cfg(target_os = "emscripten")]
mod event_loop {
    extern crate emscripten_sys;

    use piston::input::*;
    use piston::window::Window;
    use piston_window::*;
    use sdl2_window::Sdl2Window;
    use std::mem;
    use std::os::raw::c_void;
    use app::App;

    struct EventLoop {
        last_updated: f64,
        handler: fn(e: Event, app: &mut App),
        app: App,
    }

    pub fn run(handler: fn(e: Event, app: &mut App),
                  app: App) {
        unsafe {
            let mut events = Box::new(EventLoop {
                last_updated: emscripten_sys::emscripten_get_now() as f64,
                handler: handler,
                app: app,
            });
            let events_ptr = &mut *events as *mut EventLoop as *mut c_void;
            emscripten_sys::emscripten_set_main_loop_arg(Some(main_loop_c), events_ptr, 0, 1);
            mem::forget(events);
        }
    }

    extern "C" fn main_loop_c(app: *mut c_void) {
        unsafe {
            let mut events: &mut EventLoop = mem::transmute(app);
            let handler = events.handler;
            let app = &mut events.app;
            app.window.swap_buffers();

            let e: Event = AfterRenderEvent::from_after_render_args(&AfterRenderArgs, &AfterRenderArgs.into()).unwrap();
            handler(e, app);

            while let Some(input) = app.window.poll_event() {
                handler(Event::Input(input), app);
            }

            if app.window.should_close() {
                emscripten_sys::emscripten_cancel_main_loop();
                return;
            }

            let now = emscripten_sys::emscripten_get_now() as f64;
            let dt = now - events.last_updated;
            events.last_updated = now;

            let update_args = UpdateArgs { dt: dt };
            let e: Event = UpdateEvent::from_update_args(&update_args, &update_args.clone().into()).unwrap();
            handler(e, app);

            let size = app.window.size();
            let draw_size = app.window.draw_size();
            let render_args = RenderArgs {
                ext_dt: dt,
                width: size.width,
                height: size.height,
                draw_width: draw_size.width,
                draw_height: draw_size.height,
            };
            let e: Event = RenderEvent::from_render_args(&render_args, &render_args.clone().into()).unwrap();
            handler(e, app);
        }
    }
}
