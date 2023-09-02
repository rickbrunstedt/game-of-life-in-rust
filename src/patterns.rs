use crate::primitives::{Coordinate, InitialPattern};
use enum_map::{enum_map, EnumMap};

pub fn create_initial_patterns_map() -> EnumMap<InitialPattern, Vec<Coordinate>> {
    return enum_map! {
        InitialPattern::Blinker => vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 1, y: 0 },
            Coordinate { x: -1, y: 0 },
        ],
        InitialPattern::Beacon => vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: -1, y: 0 },
            Coordinate { x: -1, y: -1 },
            Coordinate { x: 0, y: -1 },
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 2, y: 2 },
        ],
        InitialPattern::Pulsar => vec![
            Coordinate { x: -6, y: -4 },
            Coordinate { x: -6, y: -3 },
            Coordinate { x: -6, y: -2 },
            Coordinate { x: -4, y: -6 },
            Coordinate { x: -3, y: -6 },
            Coordinate { x: -2, y: -6 },
            Coordinate { x: -1, y: -2 },
            Coordinate { x: -1, y: -3 },
            Coordinate { x: -1, y: -4 },
            Coordinate { x: -2, y: -1 },
            Coordinate { x: -3, y: -1 },
            Coordinate { x: -4, y: -1 },
            Coordinate { x: 1, y: -2 },
            Coordinate { x: 1, y: -3 },
            Coordinate { x: 1, y: -4 },
            Coordinate { x: 2, y: -6 },
            Coordinate { x: 3, y: -6 },
            Coordinate { x: 4, y: -6 },
            Coordinate { x: 2, y: -1 },
            Coordinate { x: 3, y: -1 },
            Coordinate { x: 4, y: -1 },
            Coordinate { x: 6, y: -2 },
            Coordinate { x: 6, y: -3 },
            Coordinate { x: 6, y: -4 },
            Coordinate { x: -2, y: 1 },
            Coordinate { x: -3, y: 1 },
            Coordinate { x: -4, y: 1 },
            Coordinate { x: -1, y: 2 },
            Coordinate { x: -1, y: 3 },
            Coordinate { x: -1, y: 4 },
            Coordinate { x: -6, y: 2 },
            Coordinate { x: -6, y: 3 },
            Coordinate { x: -6, y: 4 },
            Coordinate { x: -2, y: 6 },
            Coordinate { x: -3, y: 6 },
            Coordinate { x: -4, y: 6 },
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 1, y: 3 },
            Coordinate { x: 1, y: 4 },
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 3, y: 1 },
            Coordinate { x: 4, y: 1 },
            Coordinate { x: 6, y: 2 },
            Coordinate { x: 6, y: 3 },
            Coordinate { x: 6, y: 4 },
            Coordinate { x: 2, y: 6 },
            Coordinate { x: 3, y: 6 },
            Coordinate { x: 4, y: 6 },
        ]
    };
}
