mod tile;
mod nations;


fn main() {
    println!("Hello, world!");
    let tile = tile::LandTile::new("name".to_string(), "major city".to_string(), vec![], nations::Nations::USA);
    println!("{}", tile.to_string());
}
