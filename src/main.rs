use std::{collections::HashMap, thread, time::Duration};

/// The board size needs to be an odd number because the initial pattern
/// requires a center cell from which to calculate its offsets.
const BOARD_SIZE: usize = 5;

/// The number of times the main loop should iterate to update and render the
/// game state.
const RENDER_ITERATIONS: usize = 20;

/// The time to wait between each render.
const WAIT_TIME: Duration = Duration::from_millis(400);

type Board = Vec<Vec<bool>>;

#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Eq, Hash, PartialEq)]
enum InitialPattern {
    Blinker,
}

fn main() {
    assert_uneven_and_greater_than_one(&BOARD_SIZE);

    let mut board = create_board(BOARD_SIZE, InitialPattern::Blinker);
    let mut render_count: usize = 0;

    draw_board(&board);

    while render_count < RENDER_ITERATIONS {
        clearscreen::clear().unwrap();

        let immutable_board_clone = board.clone();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let x = x as isize;
                let y = y as isize;

                let alive_neighbours =
                    check_alive_neighbours(Coordinate { x, y }, &immutable_board_clone);

                change_state(&mut board[y as usize][x as usize], alive_neighbours);
            }
        }

        draw_board(&board);
        render_count += 1;
        thread::sleep(WAIT_TIME);
    }
}

fn change_state(cell: &mut bool, alive_neighbours: usize) {
    *cell = match (*cell, alive_neighbours) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    };
}

fn assert_uneven_and_greater_than_one(number: &usize) {
    assert!(number > &1, "Board size should be greater than 1");
    assert!(number % &2 != 0, "Board size should be an odd number");
}

fn check_alive_neighbours(coordinate: Coordinate, board: &Board) -> usize {
    let neighbour_coordinates = create_neighbour_coordinates(coordinate);

    let alive_neighbours: usize = neighbour_coordinates
        .iter()
        .filter_map(|coordinate| {
            board
                .get(coordinate.y as usize)
                .and_then(|row| row.get(coordinate.x as usize))
                .and_then(
                    |&cell_value| {
                        if cell_value {
                            Some(cell_value)
                        } else {
                            None
                        }
                    },
                )
        })
        .count();

    return alive_neighbours;
}

fn create_initial_patterns_map() -> HashMap<InitialPattern, Vec<Coordinate>> {
    let mut initial_patterns: HashMap<InitialPattern, Vec<Coordinate>> = HashMap::new();

    initial_patterns.insert(
        InitialPattern::Blinker,
        vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 1, y: 0 },
            Coordinate { x: -1, y: 0 },
        ],
    );

    return initial_patterns;
}

fn create_neighbour_coordinates(coordinate: Coordinate) -> Vec<Coordinate> {
    let mut result: Vec<Coordinate> = vec![];

    let relative_offsets: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    for offset in relative_offsets {
        result.push(Coordinate {
            x: coordinate.x + offset.0,
            y: coordinate.y + offset.1,
        })
    }

    return result;
}

fn create_board(board_size: usize, initial_pattern: InitialPattern) -> Board {
    let mut board = Vec::new();

    for _ in 0..board_size {
        let mut inner_vec = Vec::new();
        for _ in 0..board_size {
            inner_vec.push(false);
        }
        board.push(inner_vec);
    }

    let center: f32 = (board_size as f32 / 2.0).ceil();

    for coordinate in &create_initial_patterns_map()[&initial_pattern] {
        let x = (center + coordinate.x as f32 - 1 as f32) as usize;
        let y = (center + coordinate.y as f32 - 1 as f32) as usize;

        board[y][x] = true;
    }

    return board;
}

fn draw_board(board: &Board) {
    for row in board {
        for cell in row {
            if *cell == true {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!();
    }
    println!();
}
