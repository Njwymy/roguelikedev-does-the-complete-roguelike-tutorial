#[macro_use]
extern crate lazy_static;
extern crate tcod;
extern crate rand;

mod game;
mod week_01;
mod week_02;
mod week_03;
mod week_04;

fn main() {
    game::run();
}
