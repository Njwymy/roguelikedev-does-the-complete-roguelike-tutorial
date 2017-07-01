
extern crate tcod;
extern crate tcod_sys;

use std::char;

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


    let player_ascii : AsciiDrawInfo = AsciiDrawInfo{
        char:'@',
        color:colors::WHITE,
    };
    let player_tile : TileDrawInfo = TileDrawInfo::new(
        char::from_u32(258).unwrap()
    );

    let orc_ascii : AsciiDrawInfo = AsciiDrawInfo{
        char:'O',
        color:colors::GREEN,
    };
    let orc_tile : TileDrawInfo = TileDrawInfo::new(
        char::from_u32(259).unwrap()
    );

    /*
    let wall_tile = char::from_u32(256).unwrap();
    let floor_tile = char::from_u32(257).unwrap();
    let player_tile = char::from_u32(258).unwrap();
    let orc_tile = char::from_u32(259).unwrap();
    let troll_tile = char::from_u32(260).unwrap();
    let scroll_tile = char::from_u32(261).unwrap();
    let healingpotion_tile = char::from_u32(262).unwrap();
    let sword_tile = char::from_u32(263).unwrap();
    let shield_tile = char::from_u32(264).unwrap();
    let stairsdown_tile = char::from_u32(265).unwrap();
    let dagger_tile = char::from_u32(266).unwrap();
*/

/*
    let player_tile : TileDrawInfo = TileDrawInfo{
        char: char::from_u32(258).unwrap(),
        foreground:colors::WHITE,
        background:colors::BLACK,
    };
*/
    //OpenGL is needed on my mac. Otherwize it will render a white screen on startup

    //#The font has 32 chars in a row, and there's a total of 10 rows. Increase the "10" when you add new rows to the sample font file
    //libtcod.console_set_custom_font('TiledFont.png', libtcod.FONT_TYPE_GREYSCALE | libtcod.FONT_LAYOUT_TCOD, 32, 10)


    let ascii_rendering = false;

    
    let mut root = create_root(80,50, ascii_rendering);
    

/*
    let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(80, 50) // width, height
            .title("Rust/libtcod tutorial")
            .renderer(Renderer::OpenGL)     
            .init();
*/

    tcod::system::set_fps(20);
        
    let mut tick = 0;

    let mut map = Map::new(80,45);
    map.set(1,3, Tile::wall());
    map.set(1,4, Tile::wall());
    map.set(3,3, Tile::wall());
    map.set(4,3, Tile::wall());

    let mut con = Offscreen::new(map.width(), map.height());

    let player = Object::new(map.width() / 2, map.height() / 2, player_ascii, player_tile);
    let npc = Object::new(map.width() / 2 - 5, map.height() / 2, orc_ascii,orc_tile);
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

            render_all(&mut root, &mut con, &objects, &map, ascii_rendering);
        }

        //Get input / Update
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }

        tick += 1;

    }

    //We need to change which font and some other metadata based on which rendering method we are using
    fn create_root(width:i32, height:i32, ascii_rendering:bool) -> Root {

        if ascii_rendering {
            let root = Root::initializer()
                .renderer(Renderer::OpenGL)
                .size(width,height)
                .title("Rust/libtcod tutorial")
                .font("arial10x10.png", FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .init();

                return root;

        }else{
            let root = Root::initializer()
                .renderer(Renderer::OpenGL)
                .size(width,height)
                .title("Rust/libtcod tutorial")
                .font("TiledFont.png", FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .font_dimensions(32,10)
                .init();

            unsafe {
                //#The index of the first custom tile in the file
                let mut a = 256;
                for y in (5..6){
                    //#The "y" is the row index, here we load the sixth row in the font file. Increase the "6" to load any new rows from the file
                    tcod_sys::TCOD_console_map_ascii_codes_to_font(a,32,0,y);
                    a += 32;
                }  
            }

            return root;

        }
    }
    



    fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map, ascii_rendering:bool){

            //TODO:Maybe instead of branching per interation we can have a method per draw style    
            for object in objects {
                if ascii_rendering {
                    con.set_default_foreground(object.tile.foreground);
                    con.put_char(object.x, object.y, object.ascii.char, BackgroundFlag::None);
                }else{
                    con.put_char_ex(object.x,object.y,object.tile.char, object.tile.foreground, object.tile.background);
                }
            }
            con.set_default_foreground(colors::BLACK);

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
