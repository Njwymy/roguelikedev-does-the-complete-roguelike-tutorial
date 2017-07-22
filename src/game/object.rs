
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
    pub name:String,
    pub blocks:bool,
    pub alive:bool,
}

impl Object{
    pub fn new(x:i32, y:i32, ascii:AsciiDrawInfo, tile:TilesetDrawInfo, name:&str, blocks:bool) -> Self {
        Object{
            x:x,
            y:y,
            ascii:ascii,
            tile:tile,
            name:name.into(),
            blocks:blocks,
            alive:false,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn set_pos_tup(&mut self, pos:(i32, i32)) {
        self.x = pos.0;
        self.y = pos.1;
    }


    pub fn move_by(&mut self,map:&Map, dx: i32, dy: i32) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        let in_bounds = map.in_bounds(new_x,new_y);
        let not_blocked = !map.at(new_x,new_y).blocked;
    
        if not_blocked && in_bounds {
            self.x = new_x;
            self.y = new_y;   
        }
    }
}