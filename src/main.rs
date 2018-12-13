mod tile;
mod nations;
mod player;

use std::{
    //collections::HashSet,
    fs::File,
    //io,
    io::{prelude::*, BufReader},
    path::Path,
};
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
    /*let territories = */parse_territories("src/territories.txt");
   // print!("{:?}", territories);
}

fn parse_territories<P>(filename: P)
where P: AsRef<Path>
{
    let buf = BufReader::new(File::open(filename).expect("Cannot find file"));
    let mut cur_nation: Nations;
    for line in buf.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {}
        else if line.chars().next().unwrap() == ':' {
            cur_nation = nations::parse(line.get(1..).unwrap());
            println!("{}", nations::name(cur_nation));
        }
        else {
            
        }
    }
}
