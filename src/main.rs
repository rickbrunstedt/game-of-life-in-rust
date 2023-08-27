
fn main() {
    let mut board = create_board();
    board = set_initial_state(board);
    draw_board(board);
}

const BOARD_SIZE: usize = 5;
type Board = [[bool; 5]; 5];

fn create_board() -> Board  {
    return [[false; BOARD_SIZE]; BOARD_SIZE];
}

fn draw_board(board: Board) {
    for row in board {
        for col in row {
            if col {
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
    new_board[2][1] = true;
    new_board[2][2] = true;
    new_board[2][3] = true;
   return new_board; 
}
