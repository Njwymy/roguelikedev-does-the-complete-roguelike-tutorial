
use tcod::Color;
use tcod::console::*;

use week_02_ext::*;

#[derive(Debug)]
pub struct Object{
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