mod tile;
mod nations;
mod player;

use nations::Nations;
use player::Player;

fn main() {
    println!("Hello, world!");
    //setup
    let players = vec![
        Player::new(Nations::USA, 42),
        Player::new(Nations::Japan, 30),
        Player::new(Nations::SovietUnion, 24),
        Player::new(Nations::UK, 30),
        Player::new(Nations::Germany, 40)
    ];

}
