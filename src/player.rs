use nations::Nations;

#[derive(Clone, Hash)]
pub struct Player {
    pub nation: Nations,
    pub ipcs: u16,
    pub color: Color,

}
#[derive(Clone, Hash)]
pub enum Color{
    Maroon,
    Gray,
    Tan,
    Orange,
    Green,
    Neutral,
}

impl Player {
    pub fn new(nation: Nations, ipcs: u16) -> Self {
        let col = match nation {
            Nations::SovietUnion => Color::Maroon,
            Nations::Germany => Color::Gray,
            Nations::UK => Color::Tan,
            Nations::Japan => Color::Orange,
            Nations::USA => Color::Green,
            _ => Color::Neutral,
        };
        Player { nation: nation, ipcs: ipcs, color: col }
    }
}