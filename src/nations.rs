#[derive(Copy, Clone)]
pub enum Nations {
    USA,
    Japan,
    SovietUnion,
    UK,
    Germany,
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
        _ => "Unkown",
    }
}
pub fn parse(x: &str) -> Nations {
    match x {
        "USA" => Nations::USA,
        "Japan" => Nations::Japan,
        "Soviet Union" => Nations::SovietUnion,
        "UK" => Nations::UK,
        "Germany" => Nations::Germany,
        _ => Nations::Neutral,
    }
}