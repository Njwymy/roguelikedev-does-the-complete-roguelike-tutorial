


#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
}

impl Tile {

    pub fn is_wall(self) -> bool {
        return self.block_sight && self.block_sight;
    }

    pub fn empty() -> Self {
        Tile{blocked: false, block_sight: false, explored: false}
    }

    pub fn wall() -> Self {
        Tile{blocked: true, block_sight: true, explored: false}
    }
}