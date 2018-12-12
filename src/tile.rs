use nations;
use nations::Nations;

pub enum TileType {
    Land(LandTile),
    //Water(WaterTile),
}

pub struct LandTile {    
    pub name: String,
    pub owner: Nations,
    pub major_city: String,
    pub adjacencies: Vec<TileType>,
}
impl LandTile {
    pub fn new(name: String, major_city: String, adjacencies: Vec<TileType>) -> Self {
        LandTile { name: name, major_city: major_city, adjacencies: adjacencies, owner: Nations::Neutral }
    }
    pub fn new_owner(name: String, major_city: String, adjacencies: Vec<TileType>, owner: Nations) -> Self {
        LandTile { name: name, major_city: major_city, adjacencies: adjacencies, owner: owner }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(nations::name(self.owner));
        output
    }
}
/*
pub struct WaterTile {
    pub adjacencies: Vec<Tile>,
    pub number: u8,
}

pub trait WaterTile {
   // fn new(number: u8) -> Self;
    fn to_string(&self) -> String {
        String::new()
    }
}

impl WaterTile {
    /*fn new(number: u8) -> WaterTile {

    }*/
}*/