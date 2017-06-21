
extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

fn main() {

    //OpenGL is needed on my mac. Otherwize it will render a white screen on startup
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .renderer(Renderer::OpenGL)     
        .init();
    tcod::system::set_fps(LIMIT_FPS);
        

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.print(1, 1, "hello libtcod rust");
        root.flush();
        root.wait_for_keypress(true);
    }

}
