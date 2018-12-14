use nations::Nations;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    Land(LandTile),
    Water(WaterTile),
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct LandTile {    
    pub name: String,
    pub owner: Nations,
    pub major_city: String,
    pub adjacencies: Vec<Tile>,
    pub economy: u8,
}
impl LandTile {
    pub fn new(name: String, major_city: String, adjacencies: Vec<Tile>, owner: Nations, economy: u8) -> Self {
        LandTile { name: name, major_city: major_city, adjacencies: adjacencies, owner: owner, economy: economy }
    }
}/*
impl Hash for LandTile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.major_city.hash(state);
        self.economy.hash(state);
    }
}
impl Eq for LandTile {
    fn eq(&self, other: &LandTile) -> bool {
        self.pid == other.pid
    }
}*/

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct WaterTile {
    pub number: u8,
    pub adjacencies: Vec<Tile>,    
}
impl WaterTile {
    pub fn new(number: u8, adjacencies: Vec<Tile>) -> Self {
        WaterTile { number: number, adjacencies: adjacencies }
    }
}
