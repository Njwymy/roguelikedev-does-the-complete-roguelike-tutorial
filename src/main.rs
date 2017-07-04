#[macro_use]
extern crate lazy_static;
extern crate tcod;
extern crate rand;

mod week_01;
mod week_02;
mod game;
mod week_03;

fn main() {
    game::run();
}
