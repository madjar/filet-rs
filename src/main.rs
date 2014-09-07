#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate input;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate collections;
extern crate debug;

use sdl2_game_window::WindowSDL2;
use piston::{
    EventIterator,
    EventSettings,
    WindowSettings,
};

mod game;
mod graph;
mod ordfloat;


fn main() {
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "Filet".to_string(),
            size: [600, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let game_iter_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let game_iter = EventIterator::new(&mut window, &game_iter_settings);
    game::play(game_iter, 600., 600.);
}
