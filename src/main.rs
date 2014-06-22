#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate collections;

use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
};

mod game;
mod graph;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Filet".to_string(),
            size: [600, 600],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let game_iter = GameIterator::new(&mut window, &game_iter_settings);
    game::play(game_iter);
}
