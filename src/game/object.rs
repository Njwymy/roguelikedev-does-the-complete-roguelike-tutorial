
use tcod::Color;
use tcod::colors;

use tcod::console::*;

use game::*;


#[derive(Debug)]
pub struct Object{
    pub x:i32,
    pub y:i32,
    pub ascii:AsciiDrawInfo,
    pub tile:TilesetDrawInfo,
}

impl Object{
    pub fn new(x:i32, y:i32, ascii:AsciiDrawInfo, tile:TilesetDrawInfo) -> Self {
        Object{
            x:x,
            y:y,
            ascii:ascii,
            tile:tile,
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
}