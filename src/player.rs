use nations::Nations;

pub struct Player {
    pub nation: Nations,
    pub ipcs: u16,
    pub color: &'static str,

}
impl Player {
    pub fn new(nation: Nations, ipcs: u16) -> Self {
        Player { nation: nation, ipcs: ipcs,
            color: match nation {
                Nations::SovietUnion => "Maroon",
                Nations::Germany => "Gray",
                Nations::UK => "Tan",
                Nations::Japan => "Orange",
                Nations::USA => "Green",
                _ => "Neutral",
            }
        }
    }
}