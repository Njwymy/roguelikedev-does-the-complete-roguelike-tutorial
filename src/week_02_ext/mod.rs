
extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::Color;
use tcod::input::Key;
use tcod::input::KeyCode::*;

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

        if !map.at(new_x, new_y).blocked {
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


//TODO just for fun, make this one vec 
struct Map {
    tiles: Vec<Tile>,
    width:i32,
    height:i32,
}


//TODO Maybe one day we can implement an iterator over the map
//that will give the (x,y) coord of the tile and the tile itself
impl Map {
    //We use i32's for the map's width / height because
    //easier intergration with libtcod
    //less wonky math when dealing with negatives
    
    //Real question do we want to allow maps of size 0?
    pub fn new(width:i32, height:i32) -> Self {
        assert!(width >= 0, "width must be greater than or 0!");
        assert!(height >= 0, "height must be greater than or 0!");

        //
        Map {
            tiles: vec![Tile::empty(); (height * width) as usize],
            width:width,
            height:height,
        }
    }

    fn index_at(&self, x:i32, y:i32) -> usize {
        return (y * self.width() + x) as usize;

    }
    fn at(&self, x:i32, y:i32) -> &Tile {
        &self.tiles[self.index_at(x,y)]
    }

    fn set(&mut self, x:i32, y:i32, tile:Tile){
        let index = self.index_at(x,y);
        self.tiles[index] = tile;
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}


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
