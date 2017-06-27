

use week_02_ext::tile::*;

pub struct Map {
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
    pub fn at(&self, x:i32, y:i32) -> &Tile {
        &self.tiles[self.index_at(x,y)]
    }

    pub fn set(&mut self, x:i32, y:i32, tile:Tile){
        let index = self.index_at(x,y);
        self.tiles[index] = tile;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}