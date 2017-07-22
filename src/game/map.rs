#![allow(dead_code)]
extern crate rand;


use rand::Rng;

use std::cmp;

use game::rect::*;
use game::tile::*;
use game::object::*;
use game::draw_info::*;

use game::is_blocked;


pub struct Map {
    tiles: Vec<Tile>,
    width:i32,
    height:i32,
    out_of_bounds_tile: Tile,
}

//TODO Maybe one day we can implement an iterator over the map
//that will give the (x,y) coord of the tile and the tile itself
impl Map {
    //We use i32's for the map's width / height because
    //easier intergration with libtcod
    //less wonky math when dealing with negatives
    
    pub fn new(width:i32, height:i32, default_tile:Tile) -> Self {
        assert!(width > 0, "width must be greater than 0!");
        assert!(height > 0, "height must be greater than 0!");

        Map {
            tiles: vec![default_tile; (height * width) as usize],
            width:width,
            height:height,
            out_of_bounds_tile: Tile::wall(),
        }
    }

    pub fn in_bounds(&self, x:i32, y:i32) -> bool {
        x >= 0 
        && y >= 0
        && x < self.width()
        && y < self.height()
    }

    fn index_at(&self, x:i32, y:i32) -> usize {
        return (y * self.width() + x) as usize;

    }
    pub fn at(&self, x:i32, y:i32) -> &Tile {
        if !self.in_bounds(x,y) {
            return &self.out_of_bounds_tile;
        }

        &self.tiles[self.index_at(x,y)]
    }

    pub fn at_mut(&mut self, x:i32, y:i32) -> &mut Tile {
        let index = self.index_at(x,y);
        &mut self.tiles[index]
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


    fn create_room(&mut self, room: Rect, ) {
        for x in (room.x1 + 1) .. room.x2 {
            for y in (room.y1 + 1) .. room.y2 {
                self.set(x,y,Tile::empty());
            }
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            self.set(x,y, Tile::empty());
        }
    }
    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            self.set(x,y, Tile::empty());
        }
    }




    pub fn create_random_rooms(width:i32, height:i32, objects:&mut Vec<Object>) -> (Self, (i32,i32)){
        const ROOM_MAX_SIZE: i32 = 10;
        const ROOM_MIN_SIZE: i32 = 6;
        const MAX_ROOMS: i32 = 40;

        //set everything to a wall first.
        let mut map = Map::new(width,height, Tile::wall());

        //our starting position will be in the first valid room's center.
        let mut starting_position = (0, 0);

        //Then "carve" the empty rooms out.
        let mut rooms = vec![];

        //save local copy of thread_rng. Mostly for readability
        let mut rng = rand::thread_rng();

        for _ in 0..MAX_ROOMS {
            // random width and height
            let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = rng.gen_range(0, map.width() - w);
            let y = rng.gen_range(0, map.height() - h);
            let new_room = Rect::new(x, y, w, h);

            // run through the other rooms and see if they intersect with this one
            let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

            // this means there are no intersections, so this room is valid
            if !failed {
                // "carve" it to the map's tiles
                map.create_room(new_room);

                //TODO just for the hell of it make it so the player spawns randomly in the first room.
                let (new_x, new_y) = new_room.center();

                Map::place_objects(new_room, objects);
                
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
                        map.create_h_tunnel(prev_x, new_x, prev_y);
                        map.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        // first move vertically, then horizontally
                        map.create_v_tunnel(prev_y, new_y, prev_x);
                        map.create_h_tunnel(prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }   
        }
        (map, starting_position)
    }

    pub fn place_objects(room: Rect, objects: &mut Vec<Object>) {
        let MAX_ROOM_MONSTERS = 3;

        // choose random number of monsters
        let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

        for _ in 0..num_monsters {
            // choose random spot for this monster
            let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
            let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

            let mut monster = if rand::random::<f32>() < 0.8 {  // 80% chance of getting an orc
                // create an orc
                Object::new(x, y, ascii::orc, *tileset::orc,"orc", true)
            } else {
                Object::new(x, y, ascii::troll, *tileset::troll,"troll", true)
            };
            monster.alive = true;
            objects.push(monster);
        }
    }



    //followed
    //https://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664
    pub fn create_caves(width:i32, height:i32, objects:&mut Vec<Object>) -> Self {

        //set everything to a wall first.
        let mut map = Map::new(width,height, Tile::wall());

        let mut rng = rand::thread_rng();

        let chance_to_be_empty = 0.46;

        for tile in map.tiles.iter_mut(){
            let chance = rng.gen::<f32>();
            if chance < chance_to_be_empty {
                *tile = Tile::empty(); 
            }   
        }
        let sim_steps = 6;
        for _ in 0 .. sim_steps {
            map.caves_sim_step();
        }

        let max_spawn_chances = 200;
        let mut spawn_attempts = 0;
        
        let desired_monsters = 15;
        let mut spawn_amount = 0;
        

        while spawn_attempts < max_spawn_chances && spawn_amount <= desired_monsters {
            let x = rng.gen_range(0, map.width());
            let y = rng.gen_range(0, map.height());

            let tile_blocked = is_blocked(x,y, &map, objects);

            if !tile_blocked {
                
                let mut monster = if rand::random::<f32>() < 0.8 {  // 80% chance of getting an orc
                // create an orc
                    Object::new(x, y, ascii::orc, *tileset::orc,"orc", true)
                } else {
                    Object::new(x, y, ascii::troll, *tileset::troll,"troll", true)
                };
                monster.alive = true;
                objects.push(monster);
                spawn_amount +=1;   
            }
            spawn_attempts +=1;
        }

        println!("spawn amount: {} spawn_attempts: {}", spawn_amount, spawn_attempts);

     map

    }

    fn caves_sim_step(&mut self) {
        //We need to create a new map since updating the map in place will cause wonky behaviours.
        //TODO from a memory perspective we could just use boolean values to represent the walls
        //this will save memory from the map allocations
        //or... maybe just have 2 maps at a given time and free the last map once we are done with it.
        //arena allocator as well!

        let mut new_map = Map::new(self.width, self.height, Tile::wall());

        let death_limit = 3;
        let birth_limit = 4;

        for x in 0 .. self.width {
            for y in 0 .. self.height {
                let empty_neighbor_count = self.count_empty_neighbours(x,y);
                //The new value is based on our simulation rules
                
                //First, if a cell is empty but has too few neighbours, fill
                if !self.at(x,y).is_wall() {
                    if empty_neighbor_count < death_limit {
                        new_map.set(x,y, Tile::wall());
                    }
                    else{
                        new_map.set(x,y, Tile::empty());
                    }
                }
                else{
                     //Otherwise, if the cell is filled now, check if it has the right number of neighbours to be cleared
                    if empty_neighbor_count > birth_limit {
                        new_map.set(x,y, Tile::empty());
                    }
                    else{
                        new_map.set(x,y, Tile::wall());
                    }
                }
            }
        }

        *self = new_map;
    }

    //We should create a unit test for this..

    pub fn count_empty_neighbours(&self, x:i32, y:i32) -> i32{
        let mut count = 0;

        for i in -1 .. 2 {
            for j in -1 .. 2 {
                let neighbour_x = x + i;
                let neighbour_y = y + j;
                //if we're looking at the middle point do nothing
                if i ==  0 && j == 0 {}
                else if neighbour_x < 0 || neighbour_y < 0 || neighbour_x >= self.width() || neighbour_y >= self.height() {
                    //Out of bounds. Count as a neighbor?
                    count += 1;
                }else if !self.at(neighbour_x, neighbour_y).is_wall() {
                    count += 1;
                }
            }
        }
        count
    }



   

    

}