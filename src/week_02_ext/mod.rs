
extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;

mod object;
mod tile;
mod map;

use week_02_ext::object::*;
use week_02_ext::tile::*;
use week_02_ext::map::*;


const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };


pub fn run() {

    //OpenGL is needed on my mac. Otherwize it will render a white screen on startup
    let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(80, 50) // width, height
            .title("Rust/libtcod tutorial")
            .renderer(Renderer::OpenGL)     
            .init();

    tcod::system::set_fps(20);
        
    let mut tick = 0;

    let mut map = Map::new(80,45);
    map.set(1,3, Tile::wall());
    map.set(1,4, Tile::wall());
    map.set(3,3, Tile::wall());
    map.set(4,3, Tile::wall());

    let mut con = Offscreen::new(map.width(), map.height());

    let player = Object::new(map.width() / 2, map.height() / 2, '@', colors::WHITE);
    let npc = Object::new(map.width() / 2 - 5, map.height() / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    //Typically a game loop is considered to be 
    //Get Input, Update Logic, Render
    //But since we are a turn based and only render after we get input
    //we Render first, then Get Input then Update Logical.
    while !root.window_closed(){

        //Render
        {
            con.set_default_foreground(colors::WHITE);
            con.print(1, 1, tick.to_string());

            render_all(&mut root, &mut con, &objects, &map);
        }

        //Get input / Update
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }

        tick += 1;

    }

    fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map){
            for object in objects {
                object.draw(con);
            }

            for x in 0 .. map.width() {
                for y in 0 .. map.height() {
                    let wall = map.at(x,y).block_sight;
                    let (x,y) = (x as i32, y as i32); //convert from index type usize to map type i32

                    if wall {
                        con.set_char_background(x,y,COLOR_DARK_WALL,BackgroundFlag::Set);
                    } else {
                        con.set_char_background(x,y,COLOR_DARK_GROUND, BackgroundFlag::Set);

                    }
                }
            }

            blit(
                //from
                con, (0, 0), (con.width(), con.height()), 
                //to
                root, (0, 0), 1.0, 1.0);
            root.flush();
            
            con.clear();
    }

    fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
        
        //real time
        //I'm thinking if you want animations outside of a turnbased game this is what you 
        //would need. (plus emulating your own simulation ticks. (your own turned based code))
        /*
        let key = match root.check_for_keypress(tcod::input::KEY_PRESSED){
            Some(key) => key,
            None => return false,
        };
        */

        //turn based
        let key = root.wait_for_keypress(true);

        match key {
            Key{code: Up, ..} => player.move_by(map,0,-1),
            Key{code: Down, ..} => player.move_by(map,0,1),
            Key{code: Left, ..} => player.move_by(map,-1,0),
            Key{code: Right, ..} => player.move_by(map,1,0),
            
            Key{code: Enter, alt:true, ..} => {
                let currently_fullscreen = root.is_fullscreen();
                root.set_fullscreen(!currently_fullscreen);
            },
            Key{code: Escape, ..} => {
                return true
            }
            _ => {}, 
        }

        false
    }


}
