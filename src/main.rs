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
    let board = build_board("src/adjacencies.txt");
    println!("{}", board.len());
}

fn parse_territories<P>(filename: P) -> Vec<Tile>
where P: AsRef<Path>
{
    let buf = BufReader::new(File::open(filename).expect("Cannot find file"));
    let re = Regex::new(r#"^"(?P<name>.+)"\s(?P<value>\d*)(\s"(?P<capital>.+)")?$"#).unwrap();
    let mut cur_nation: Nations = Nations::UK;
    let mut territories: Vec<Tile> = vec!();

    for line in buf.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {}
        else if line.chars().next().unwrap() == ':' {
            cur_nation = nations::parse(line.get(1..).unwrap());
        }
        else {
            let caps = re.captures(line.as_str()).unwrap();
            let value: u8 = caps.name("value").unwrap().as_str().parse().unwrap();
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
fn build_board<P>(filename: P) -> HashMap<Tile, Vec<Tile>>
where P: AsRef<Path>
{
    let buf = BufReader::new(File::open(filename).expect("Cannot find file"));
    //let re_land = Regex::new(r#""(.*?)""#).unwrap();
    let re_sea = Regex::new(r#"\d+?"#).unwrap();
    let mut map: HashMap<Tile, Vec<Tile>> = HashMap::new();
    let territories: Vec<Tile> = parse_territories("src/territories.txt");
    
    for line in buf.lines().map(|l| l.unwrap()) {
        let mut adjs: Vec<Tile> = vec!();
        let mut name: String = String::new();

        if line.len() == 0 {}
        else {
            let mut first = true;
            for mat in Regex::new(r#""(.*?)""#).unwrap().captures_iter(line.as_str()) {
                if first {
                    name = mat[1].to_string();
                    first = false;
                }
                else {
                    let name = mat[1].to_string();
                    let ter: Tile = territories.iter().find(|t|{
                        match t {
                            Tile::Land(a) => a.name == name,
                            Tile::Water(a) => false,
                        }
                    }).unwrap().clone();
                }
            }
        }
        println!("{}", adjs.len());
        //map.insert(name, adjs);
    }
    map
}
