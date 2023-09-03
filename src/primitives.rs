use clap::{Parser, ValueEnum};
use enum_map::Enum;

pub type Board = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Eq, Hash, Clone, PartialEq, Enum, ValueEnum)]
pub enum InitialPattern {
    Beacon,
    Blinker,
    Pulsar,
}

/// Conway's Game of Life
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub pattern: InitialPattern,
}
