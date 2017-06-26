#![allow(dead_code)]
extern crate tcod;

use std::iter::Iterator;

use tcod::console::*;
use tcod::colors;
use tcod::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32= 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

#[derive(Debug)]
struct Object{
    x:i32,
    y:i32,
    char:char,
    color:Color
}

impl Object{
    pub fn new(x:i32, y:i32, char:char, color:Color) -> Self {
        Object{
            x:x,
            y:y,
            char:char,
            color:color,
        }
    }

    pub fn move_by(&mut self,map:&Map, dx: i32, dy: i32) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if !map[new_x as usize][new_y as usize].blocked {
            self.x = new_x;
            self.y = new_y;   
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile{blocked: false, block_sight: false}
    }

    pub fn wall() -> Self {
        Tile{blocked: true, block_sight: true}
    }
}


//no
type Map = Vec<Vec<Tile>>;

fn make_map() -> Map {
    let map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map
}


pub fn run() {

    //OpenGL is needed on my mac. Otherwize it will render a white screen on startup
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .renderer(Renderer::OpenGL)     
        .init();
    
    tcod::system::set_fps(LIMIT_FPS);
        
    let mut tick = 0;
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let player = Object::new(MAP_WIDTH / 2, MAP_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new(MAP_WIDTH / 2 - 5, MAP_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    let mut map = make_map();
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();



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

            for (x,row) in map.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    
                    let wall = cell.block_sight;
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
