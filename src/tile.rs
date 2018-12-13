use nations::Nations;

pub enum Tile {
    Land(LandTile),
    Water(WaterTile),
}

pub struct LandTile {    
    pub name: String,
    pub owner: Nations,
    pub major_city: String,
    pub adjacencies: Vec<Tile>,
}
impl LandTile {
    pub fn new(name: String, major_city: String, adjacencies: Vec<Tile>, owner: Nations) -> Self {
        LandTile { name: name, major_city: major_city, adjacencies: adjacencies, owner: owner }
    }
}

pub struct WaterTile {
    pub number: u8,
    pub adjacencies: Vec<Tile>,    
}
impl WaterTile {
    pub fn new(number: u8, adjacencies: Vec<Tile>) -> Self {
        WaterTile { number: number, adjacencies: adjacencies }
    }
}
