extern crate log;

use ggez::{event, ContextBuilder};
use ggez::conf::WindowSetup;

mod game;
pub mod common;

use game::Game;

fn main() {
    common::logger::init().unwrap();

    let window_setup = WindowSetup::default().title("");

    let (mut ctx, event_loop) = ContextBuilder::new("", "mertwole")
        .window_setup(window_setup)
        .build()
        .expect("could not create ggez context!");

    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}