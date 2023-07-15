const UNASSIGNED: i32 = 0; // An unassigned cell in the Sudoku
const SIZE: usize = 9; // The size of the Sudoku board

fn is_valid(board: &[[i32; SIZE]; SIZE], row: usize, col: usize, num: i32) -> bool {
    // Check if the number already exists in the current row
    for x in 0..SIZE {
        if board[row][x] == num {
            return false;
        }
    }

    // Check if the number exists in the current column
    for x in 0..SIZE {
        if board[x][col] == num {
            return false;
        }
    }

    // Check if the number exists in the current 3x3 box
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }

    true
}

fn solve_sudoku(board: &mut [[i32; SIZE]; SIZE]) -> bool {
    let mut row = 0;
    let mut col = 0;
    let mut check_empty = false;

    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] == UNASSIGNED {
                row = i;
                col = j;
                check_empty = true;
                break;
            }
        }
        if check_empty {
            break;
        }
    }

    // No empty spaces left, the board is solved
    if !check_empty {
        return true;
    }

    for num in 1..=SIZE as i32 {
        if is_valid(board, row, col, num) {
            board[row][col] = num;
            if solve_sudoku(board) {
                return true;
            }
            board[row][col] = UNASSIGNED;
        }
    }

    false
}

fn print_board(board: &[[i32; SIZE]; SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let s = String::new();
    let mut board: [[i32; SIZE]; SIZE] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];

    if solve_sudoku(&mut board) {
        print_board(&board);
    } else {
        println!("No solution exists");
    }
}
