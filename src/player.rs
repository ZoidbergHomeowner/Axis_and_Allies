use nations::Nations;

#[derive(Clone, Hash)]
pub struct Player {
    pub nation: Nations,
    pub ipcs: u16,
    pub color: String,

}
impl Player {
    pub fn new(nation: Nations, ipcs: u16) -> Self {
        let col = String::from( match nation {
            Nations::SovietUnion => "Maroon",
            Nations::Germany => "Gray",
            Nations::UK => "Tan",
            Nations::Japan => "Orange",
            Nations::USA => "Green",
            _ => "Neutral",
        });
        Player { nation: nation, ipcs: ipcs, color: col }
    }
}