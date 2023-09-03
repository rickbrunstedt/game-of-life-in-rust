mod patterns;
mod primitives;

use crate::patterns::create_patterns_map;
use crate::primitives::{Args, Board, Coordinate, InitialPattern};
use clap::Parser;
use std::{thread, time::Duration};

/// The board size needs to be an odd number because the initial pattern
/// requires a center cell from which to calculate its offsets.
pub const BOARD_SIZE: usize = 15;

/// The number of times the main loop should iterate to update and render the
/// game state.
const RENDER_ITERATIONS: usize = 20;

/// The time to wait between each render.
const WAIT_TIME: Duration = Duration::from_millis(500);

fn main() {
    validate_board_size(&BOARD_SIZE);
    let initial_pattern = Args::parse().pattern;

    let mut board = create_board(BOARD_SIZE, initial_pattern);
    let mut render_count: usize = 0;

    draw_board(&board);

    while render_count < RENDER_ITERATIONS {
        // Needed because the state of each cell must be calculated from the
        // initial board state during each iteration.
        let immutable_board_clone = board.clone();

        clear_screen();

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

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn change_state(cell: &mut bool, alive_neighbours: usize) {
    *cell = match (*cell, alive_neighbours) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    };
}

fn validate_board_size(number: &usize) {
    assert!(number > &1, "Board size should be greater than 1");
    assert!(number % &2 != 0, "Board size should be an odd number");
}

fn check_alive_neighbours(coordinate: Coordinate, board: &Board) -> usize {
    create_neighbour_coordinates(coordinate)
        .iter()
        .filter(|&coord| {
            *board
                .get(coord.y as usize)
                .and_then(|row| row.get(coord.x as usize))
                .unwrap_or(&false)
        })
        .count()
}

fn create_neighbour_coordinates(coordinate: Coordinate) -> Vec<Coordinate> {
    let relative_offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    return relative_offsets
        .iter()
        .map(|&(dx, dy)| Coordinate {
            x: coordinate.x + dx,
            y: coordinate.y + dy,
        })
        .collect();
}

fn create_board(board_size: usize, initial_pattern: InitialPattern) -> Board {
    let mut board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];
    let center = (board_size as f32 / 2.0).ceil();

    for coordinate in &create_patterns_map()[initial_pattern] {
        let x = (center + coordinate.x as f32 - 1.0) as usize;
        let y = (center + coordinate.y as f32 - 1.0) as usize;
        board[y][x] = true;
    }

    return board;
}

fn draw_board(board: &Board) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { "O" } else { " " });
        }
        println!();
    }
}
