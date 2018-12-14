extern crate regex;

mod tile;
mod nations;
mod player;

use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;
use nations::Nations;
use player::Player;
use tile::Tile;

fn main() {
    println!("Hello, world!");
    //setup
    let _players = vec![
        Player::new(Nations::USA, 42),
        Player::new(Nations::Japan, 30),
        Player::new(Nations::SovietUnion, 24),
        Player::new(Nations::UK, 30),
        Player::new(Nations::Germany, 40)
    ];
    let territories = parse_territories("src/territories.txt");
    println!("{}", territories.len());
}

fn parse_territories<P>(filename: P) -> Vec<Tile>
where P: AsRef<Path>
{
    let buf = BufReader::new(File::open(filename).expect("Cannot find file"));
    let re = Regex::new(r#"^"(?P<name>.+)"\s(?P<value>\d*)(\s"(?P<capital>.+)")?$"#).unwrap();
    let mut cur_nation: Nations = Nations::UK;
    let mut territories: Vec<Tile> = vec!();
    let asdf = parse_adjacencies("src/adjacencies.txt");

    for line in buf.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {}
        else if line.chars().next().unwrap() == ':' {
            cur_nation = nations::parse(line.get(1..).unwrap());
        }
        else {
            let caps = re.captures(line.as_str()).unwrap();
            let value: u8 = caps.name("value").unwrap().as_str().parse().unwrap();
            //let adjacencies: parse_adjacencies("src/adjacencies.txt");
            let adjacencies: Vec<Tile> = vec!();
            
            territories.push(Tile::Land(tile::LandTile::new(
                String::from(caps.name("name").unwrap().as_str()),
                String::from(if caps.name("capital") != None {caps.name("capital").unwrap().as_str()} else {""}),
                adjacencies,
                cur_nation.clone(),
                value
            )));
        }
    }
    territories
}
fn parse_adjacencies<P>(filename: P) -> HashMap<Tile, Vec<Tile>>
where P: AsRef<Path>
{
    let re_land = Regex::new(r#""(.*?)""#).unwrap();
    let re_sea = r#""#;
    let buf = BufReader::new(File::open(filename).expect("Cannot find file"));
    let tile_type = Tile::Land;
    
    for line in buf.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {}
        else {
            let caps = re_land.captures(line.as_str()).unwrap();
            let name = String::from(caps.get(1).unwrap().as_str());
            println!("name: {}", name);
            let mut adj: Vec<String> = vec!();
            let n: usize = 2;
            while caps.get(n) != None {
                adj.push(String::from(caps.get(n).unwrap().as_str()));
                println!("{}", caps.get(n).unwrap().as_str());
            }
        }
    }
    let asdf: HashMap<Tile, Vec<Tile>> = HashMap::new();
    asdf
}