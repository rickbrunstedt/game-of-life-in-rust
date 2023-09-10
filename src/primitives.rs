use crate::BOARD_SIZE;
use clap::{Parser, ValueEnum};
use enum_map::Enum;

pub type Board = [[bool; BOARD_SIZE]; BOARD_SIZE];

#[derive(Eq, Hash, Clone, PartialEq, Enum, ValueEnum)]
pub enum InitialPattern {
    Beacon,
    Blinker,
    Pulsar,
}

pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

// CLI configuration
/// Conway's Game of Life
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub pattern: InitialPattern,
}
