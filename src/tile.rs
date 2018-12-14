use nations::Nations;

#[derive(Clone)]
pub enum Tile {
    Land(LandTile),
    Water(WaterTile),
}

#[derive(Clone)]
pub struct LandTile {    
    pub name: &'static str,
    pub owner: Nations,
    pub major_city: &'static str,
    pub adjacencies: Vec<Tile>,
    pub economy: u8,
}
impl LandTile {
    pub fn new(name: &'static str, major_city: &'static str, adjacencies: Vec<Tile>, owner: Nations, economy: u8) -> Self {
        LandTile { name: name, major_city: major_city, adjacencies: adjacencies, owner: owner, economy: economy }
    }
}

#[derive(Clone)]
pub struct WaterTile {
    pub number: u8,
    pub adjacencies: Vec<Tile>,    
}
impl WaterTile {
    pub fn new(number: u8, adjacencies: Vec<Tile>) -> Self {
        WaterTile { number: number, adjacencies: adjacencies }
    }
}
