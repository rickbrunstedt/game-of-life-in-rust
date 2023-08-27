fn main() {
    let mut board = create_board();
    board = set_initial_state(board);
    draw_board(board);
    let theone = board[2][1];

    // dbg!(theone.alive);
    // dbg!(theone.alive);
    // assert!(board[2][3].alive == false);
    dbg!(board[2][1]);
    theone.check_and_change_state(Coordinate { x: 2, y: 1 }, &board);
    dbg!(board[2][1]);
    // dbg!(theone.alive);
    //assert!(board[2][3].alive == true);
}

const BOARD_SIZE: usize = 5;
type Board = [[Cell; 5]; 5];

#[derive(Copy, Clone, PartialEq, Debug)]
struct Cell {
    alive: bool,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl Cell {
    fn check_and_change_state(mut self, coordinate: Coordinate, board: &Board) {
        let neighbours: Vec<bool> = vec![
            || -> bool {
                let row = board.get(coordinate.x - 1);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y - 1);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x - 1);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x - 1);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y + 1);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y - 1);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y + 1);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x + 1);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y - 1);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x + 1);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
            || -> bool {
                let row = board.get(coordinate.x + 1);
                if row.is_none() {
                    return false;
                }
                let cell = row.unwrap().get(coordinate.y + 1);
                if cell.is_none() {
                    return false;
                }
                return cell.unwrap().alive;
            }(),
        ];
        dbg!(&neighbours);

        let alive_neighbours = neighbours.iter().filter(|val| **val).count();
        if self.alive {
            if alive_neighbours > 2 || alive_neighbours < 3 {
                self.alive = false;
            } else {
                self.alive = true;
            }
        } else {
            if alive_neighbours == 3 {
                self.alive = true;
            } else {
                self.alive = false;
            }
        }
        dbg!(self.alive);
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
