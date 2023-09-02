use enum_map::Enum;

pub type Board = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

#[derive(Eq, Hash, PartialEq, Enum)]
pub enum InitialPattern {
    Blinker,
    Beacon,
    Pulsar,
}
