#![allow(dead_code)]
extern crate tcod;
extern crate rand;

use std::iter::Iterator;
use std::cmp;

use tcod::console::*;
use tcod::colors;
use tcod::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::map::{Map as FovMap, FovAlgorithm};

use rand::Rng;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32= 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

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
    explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile{blocked: false, block_sight: false, explored:false}
    }

    pub fn wall() -> Self {
        Tile{blocked: true, block_sight: true, explored:false}
    }
}

type Map = Vec<Vec<Tile>>;

fn make_map() -> (Map, (i32,i32)) {

    const ROOM_MAX_SIZE: i32 = 10;
    const ROOM_MIN_SIZE: i32 = 6;
    const MAX_ROOMS: i32 = 40;

    //set everything to a wall first.
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    //our starting position will be in the first valid room's center.
    let mut starting_position = (0, 0);

    //Then "carve" the empty rooms out.
    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);
        let new_room = Rect::new(x, y, w, h);

        // run through the other rooms and see if they intersect with this one
        let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        // this means there are no intersections, so this room is valid
        if !failed {
            // "carve" it to the map's tiles
            create_room(new_room, &mut map);

            //TODO just for the hell of it make it so the player spawns randomly in the first room.
            let (new_x, new_y) = new_room.center();
            
            if rooms.is_empty() {
                //First room since there isnt any other rooms
                starting_position = (new_x, new_y);
            }else{
                //Non first room. 
                // all rooms after the first:
                // connect it to the previous room with a tunnel
                // center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                // draw a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    // first move vertically, then horizontally
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room);
        }   
    }
    (map, starting_position)
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1) .. room.x2 {
        for y in (room.y1 + 1) .. room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}


#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2) && (self.x2 >= other.x1) 
        &&
        (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }
}


pub fn run() {

    //OpenGL is needed on my mac. Otherwize it will render a white screen on startup
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("week 4")
        .renderer(Renderer::OpenGL)     
        .init();
    
    tcod::system::set_fps(LIMIT_FPS);
        
    let mut tick = 0;
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);


    // generate map (at this point it's not drawn to the screen)
    let (mut map, (player_x, player_y)) = make_map();

    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            fov_map.set(x, y,
                        !map[x as usize][y as usize].block_sight,
                        !map[x as usize][y as usize].blocked);
        }
    }

    let player = Object::new(player_x, player_y, '@', colors::WHITE);
    let mut previous_player_location = (-1,-1);
    
    let mut objects = [player];

    //Typically a game loop is considered to be 
    //Get Input, Update Logic, Render
    //But since we are a turn based and only render after we get input
    //we Render first, then Get Input then Update Logical.
    while !root.window_closed(){

        //Render
        {
            con.set_default_foreground(colors::WHITE);
            con.print(1, 1, tick.to_string());

            let fov_recompute = {//for player lifetime
                let player = &mut objects[0];
                previous_player_location != (player.x, player.y)
            };

            render_all(&mut root, &mut con, &objects, &mut map, &mut fov_map, fov_recompute);
            
        }

        

        //Get input / Update
        let player = &mut objects[0];
        previous_player_location = (player.x, player.y);
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }

        tick += 1;

    }

    fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &mut Map, fov_map: &mut FovMap, fov_recompute: bool){

            if fov_recompute {
                let player = &objects[0];
                fov_map.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
            }

            for object in objects {
                if fov_map.is_in_fov(object.x,object.y){
                    object.draw(con);
                }
            }

            for (x,row) in map.iter_mut().enumerate() {
                for (y, cell) in row.iter_mut().enumerate() {

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

                    let explored = &mut cell.explored;
                    if visible {
                        // since it's visible, explore it
                        *explored = true;
                    }
                    if *explored {
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

    fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
        
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
