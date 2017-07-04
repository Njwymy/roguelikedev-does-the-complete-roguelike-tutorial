use tcod::Color;
use tcod::colors;

#[derive(Clone, Copy, Debug)]
pub struct AsciiDrawInfo {
    pub char:char,
    pub color:Color,
}

#[derive(Clone, Copy, Debug)]
pub struct TilesetDrawInfo {
    pub char:char,
    pub foreground:Color,
    pub background:Color,
}

impl TilesetDrawInfo {
    pub fn new(char:char) -> TilesetDrawInfo {
        TilesetDrawInfo{
            char:char,
            foreground:colors::WHITE,
            background:colors::BLACK,
        }
    }
}

pub mod tileset {
    use game::draw_info::TilesetDrawInfo;
    use std::char;
    use tcod::colors;

    
    //the libtcod tutorial for tilesets uses char values greater than 255 to map to specefic tiles 
    //in the supplied tileset. This causes us to need to use the from_u32() function to create chars
    //since rust doesn't really like to map numbers to char values. Because of this
    //we can't do easy static values like ascii so we have to use lazy_static! macro do this
    //This also causes a side effect of us having to deference this value anytime we use these.

    //usage *tileset::player or whatever tile you need.
    lazy_static! { 
        pub static ref player : TilesetDrawInfo = {
            TilesetDrawInfo::new(
                char::from_u32(258).unwrap()
            )
        };
        pub static ref orc : TilesetDrawInfo = {
            TilesetDrawInfo::new(
                char::from_u32(259).unwrap()
            )
        };
    }

    //reference for later tilesets
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
    
}

pub mod ascii {
    use game::draw_info::AsciiDrawInfo;
    use tcod::colors;

    pub static player : AsciiDrawInfo = AsciiDrawInfo{
        char:'@',
        color:colors::WHITE,
    };
    pub static orc : AsciiDrawInfo = AsciiDrawInfo{
        char:'O',
        color:colors::GREEN,
    };
}
