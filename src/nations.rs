pub enum Nations {
    USA,
    Germany,
    UK,
    SovietUnion,
    Japan,
    Neutral,
}

pub fn name(x: Nations) -> &'static str {
    match x {
        Nations::USA => "USA",
        Nations::Germany => "Germany",
        Nations::UK => "UK",
        Nations::SovietUnion => "Soviet Union",
        Nations::Japan => "Japan",
        Nations::Neutral => "Neutral",
    }
}