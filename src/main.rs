fn main() {
    let mut board = create_board();
    board = set_initial_state(board);
    draw_board(board);
}

const BOARD_SIZE: usize = 5;
type Board = [[Cell; 5]; 5];

#[derive(Copy, Clone, PartialEq)]
struct Cell {
    alive: bool,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl Cell {
    fn check_and_change_state(self: Self, coordinate: Coordinate, board: &Board) {
        let x: Vec<bool> = vec![|| -> bool {
            let row = board.get(coordinate.x - 1);
            if row.is_none() {
                return false;
            }
            let cell = row.unwrap().get(coordinate.y - 1);
            if cell.is_none() {
                return false;
            }
            return cell.unwrap().alive;
        }()];
    }
}

fn create_board() -> Board {
    return [[Cell { alive: false }; BOARD_SIZE]; BOARD_SIZE];
}

fn draw_board(board: Board) {
    for row in board {
        for col in row {
            if col.alive {
                print!("X");
            } else {
                print!("_");
            }
        }
        println!();
    }
}

fn set_initial_state(board: Board) -> Board {
    let mut new_board = board.clone();
    new_board[2][1].alive = true;
    new_board[2][2].alive = true;
    new_board[2][3].alive = true;
    return new_board;
}
