extern crate regex;

mod tile;
mod nations;
mod player;

use std::{
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
    let _territories = parse_territories("src/territories.txt");
    //print!("{:?}", territories);
}

fn parse_territories<P>(filename: P) -> Vec<Tile>
where P: AsRef<Path>
{
    let buf = BufReader::new(File::open(filename).expect("Cannot find file"));
    let re = Regex::new(r#"^"(?P<name>.+)"\s(?P<value>\d*)(\s"(?P<capital>.+)")?$"#).unwrap();
    let mut cur_nation: Nations = Nations::UK;
    let mut territories: Vec<Tile> = vec!();
    /*for line in buf.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {}
        else if line.chars().next().unwrap() == ':' {
            cur_nation = nations::parse(line.get(1..).unwrap());
            println!("{}", nations::name(cur_nation));
        }
        else {
            let caps = re.captures(line.as_str()).unwrap();
            let name = caps.name("name").unwrap().as_str();
            let value: u8 = caps.name("value").unwrap().as_str().parse().unwrap();
            let capital: &str = if caps.name("capital") != None { caps.name("capital").unwrap().as_str() } else { "" };
            let adjacencies: Vec<Tile> = vec!();
            
            territories.push(Tile::Land(tile::LandTile::new(name, capital, adjacencies, cur_nation, value)));
        }
    }*/
    let mut name: &str;
    let mut value: u8;
    let mut capital: &str;
    let mut caps: regex::Captures<'_>;
    let mut line: String;
    for l in buf.lines().map(|l| l.unwrap()) {
        line = String::from(l);
        if line.len() == 0 {}
        else if line.chars().next().unwrap() == ':' {
            cur_nation = nations::parse(line.get(1..).unwrap());
            println!("{}", nations::name(cur_nation));
        }
        else {
            caps = re.captures(line.as_str()).unwrap();
            name = caps.name("name").unwrap().as_str();
            value = caps.name("value").unwrap().as_str().parse().unwrap();
            capital = if caps.name("capital") != None { caps.name("capital").unwrap().as_str() } else { "" };
            let adjacencies: Vec<Tile> = vec!();
            
            territories.push(Tile::Land(tile::LandTile::new(name, capital, adjacencies, cur_nation, value)));
        }
    }
    territories
}
