
extern crate tcod;
extern crate tcod_sys;
extern crate rand;

use tcod::console::*;
use tcod::colors;
use tcod::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::map::{Map as FovMap, FovAlgorithm};

mod object;
mod tile;
mod map;
mod draw_info;
mod rect;

use game::object::*;
use game::map::*;
use game::draw_info::*;
use game::tile::*;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };


pub fn run() {
    
    let ascii_rendering = true;
    let mut root = create_root(80,50, ascii_rendering);
    tcod::system::set_fps(20);
        
    let mut tick = 0;

    //let (map, (start_x, start_y)) = Map::create_random_rooms(80,45);

    let (start_x, start_y) = (0,0);
    //let mut map = Map::create_caves(80,45);
    let mut map = Map::new(80,45, Tile::empty());

    let mut fov_map = FovMap::new(map.width(), map.height());
    for y in 0..map.height() {
        for x in 0..map.width() {
            let cell = map.at(x,y);
            fov_map.set(x, y,!cell.block_sight,!cell.blocked);
        }
    }


    let mut con = Offscreen::new(map.width(), map.height());

    let player = Object::new(start_x, start_y, ascii::player, *tileset::player);
    //start at a impossible value so we can trigger any sort of "player is at a diffrent location"
    //logic on the first turn.
    let mut previous_player_location = (-1,-1); 

    let npc = Object::new(start_x + 1, start_y + 1, ascii::orc, *tileset::orc);
    let mut objects = [player, npc];

    //Typically a game loop is considered to be 
    //Get Input, Update Logic, Render
    //But since we are a turn based and only render after we get input
    //we Render first, then Get Input then Update Logical.
    while !root.window_closed(){

        const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
        const FOV_LIGHT_WALLS: bool = true;
        const TORCH_RADIUS: i32 = 10;

        //While this should be in the update step
        //we need this to be true for the first tick.
        //probably need to stort this out so it can be in the update area.
        let fov_recompute = {//scope for player lifetime
            let player = &mut objects[0];
            previous_player_location != (player.x, player.y)
        };
        //compute the fov before the first tick so the user can see something.
        //Update fov / explored cells
        if fov_recompute {
            let player = &objects[0];
            fov_map.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
        }
        for x in 0 .. map.width() {
            for y in 0 .. map.height() {
                let cell = map.at_mut(x,y);
                let (x,y) = (x as i32, y as i32); //convert from index type usize to map type i32
                let visible = fov_map.is_in_fov(x, y);
                if visible {
                    // since it's visible, explore it
                    cell.explored = true;
                }
            }
        }



        //Render
        {
            con.set_default_foreground(colors::WHITE);
            con.print(1, 1, tick.to_string());

            render_all(&mut root, &mut con, &objects, &map, ascii_rendering, &fov_map);
        }


        //Get input / Update
        {//for player borrow scope
            let player = &mut objects[0];
            previous_player_location = (player.x, player.y);
            let exit = handle_keys(&mut root, player, &map);
            if exit {
                break;
            }
        }
        tick += 1;

    }
}

//We need to change which font and some other metadata based on which rendering method we are using
//OpenGL is needed on my mac. Otherwize it will render a white screen on startup
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
            //#The font has 32 chars in a row, and there's a total of 10 rows. Increase the "10" when you add new rows to the sample font file
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

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map, ascii_rendering:bool,fov_map: &FovMap){

    //TODO:Maybe instead of branching per interation we can have a method per draw style    
    for object in objects {
        if fov_map.is_in_fov(object.x,object.y){
            if ascii_rendering {
                con.set_default_foreground(object.tile.foreground);
                con.put_char(object.x, object.y, object.ascii.char, BackgroundFlag::None);
            }else{
                con.put_char_ex(object.x,object.y,object.tile.char, object.tile.foreground, object.tile.background);
            }
        }
    }
    con.set_default_foreground(colors::BLACK);

    for x in 0 .. map.width() {
        for y in 0 .. map.height() {
            let cell = map.at(x,y);
            let wall = cell.is_wall();
            let (x,y) = (x as i32, y as i32); //convert from index type usize to map type i32

            let visible = fov_map.is_in_fov(x, y);
            let wall = cell.block_sight;
            let color = match (visible, wall) {
                // outside of field of view:
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside fov:
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };

            if cell.explored {
                // show explored tiles only (any visible tile is explored already)
                con.set_char_background(x, y, color, BackgroundFlag::Set);
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
