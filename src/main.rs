#[macro_use]
extern crate lazy_static;
extern crate tcod;
extern crate rand;

mod week_01;
mod week_02;
mod week_02_ext;
mod week_03;

fn main() {
    week_03::run();
}
