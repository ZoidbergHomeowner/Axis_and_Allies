#[derive(Copy, Clone)]
pub enum Nations {
    USA,
    Japan,
    SovietUnion,
    UK,
    Germany,
    Neutral,
}

pub fn is_axis(x: Nations) -> bool {
    match x {
        Nations::Japan => true,
        Nations::Germany => true,
        _ => false
    }
}
pub fn is_allies(x: Nations) -> bool {
    match x {
        Nations::USA => true,
        Nations::SovietUnion => true,
        Nations::UK => true,
        _ => false
    }
}
pub fn same_team(x: Nations, y: Nations) -> bool {
    (is_allies(x) && is_allies(y)) || (is_axis(x) && is_axis(y))
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