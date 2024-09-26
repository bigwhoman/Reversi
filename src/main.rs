use std::collections::HashMap;
use std::io;
use std::io::Write;

const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;

fn main() {
    game();
}

fn game() {
    let board = initialize();
    // board[1][2] = 2;
    // print_board(board);
    // board[2][3] = 5;
    // print_board(board);
    start_game(board);
}

// Initial things -- Num of rows and columns
fn initialize() -> [[char; ROW_SIZE]; COL_SIZE] {
    let mut board: [[char; ROW_SIZE]; COL_SIZE] = [['.'; ROW_SIZE]; COL_SIZE];
    board[3][3] = 'B';
    board[4][4] = 'B';
    board[3][4] = 'W';
    board[4][3] = 'W';

    return board;
}

fn start_game(mut board: [[char; ROW_SIZE]; COL_SIZE]) {
    let mut player = 'B';
    let map: HashMap<char, usize> = ('a'..='h').zip(0..8).collect();
    loop {
        let mut input_line: String = Default::default();
        print!("Enter move for colour {} (RowCol): ", player);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_line).expect("Wrong input");

        let row_move: char = input_line.trim().chars().nth(0).unwrap();
        let col_move: char = input_line.trim().chars().nth(1).unwrap();

        if let (Some(&row), Some(&col)) = (map.get(&row_move), map.get(&col_move)) {
            if board[row][col] == '.' {
                board[row][col] = player;
                do_move(&mut board, row, col, player);
            } else {
                println!("Invalid Move");
            }
        } else {
            println!("Invalid move");
        }

        print_board(board);
        if player == 'B' {
            player = 'W';
        } else {
            player = 'B';
        }
        if check_for_end() {
            // break;
        }
    }
    // println!("Game has ended and {} has won!!!", 12);
}

fn do_move(board: &mut [[char; ROW_SIZE]; COL_SIZE], row: usize, col: usize, player:char) {
    for i in [-1_i32, 0, 1] {
        for j in [-1_i32, 0, 1] {
            let mut temp_row: i32 = row as i32;
            let mut temp_col: i32 = col as i32;
            if i == 0 && j == 0 {
                continue;
            } else {
                while ['W', 'B'].iter().any(|&x| x == board[temp_row.abs() as usize][temp_col.abs() as usize]) {
                    // println!("col {} row : {}",temp_col, temp_row);

                    temp_row += i ;
                    temp_col += j ;
                    println!("tmp row {} col : {}",temp_row, temp_col);
                    println!("i {} and j {}", i, j);
                    if board[temp_row.abs() as usize][temp_col.abs() as usize] == player {
                        println!("nooooooogi");
                        println!("lampppppppppppp row {} col : {}",temp_row, temp_col);
                        println!("jampppppp row {} col : {}",row as i32, col as i32);
                        while temp_row != row as i32 || temp_col != col as i32 {
                            temp_col -= j ;
                            temp_row -= i ;
                            
                            println!("shooo gi ---> row {} col{}", temp_row, temp_col);
                            board[temp_row.abs() as usize][temp_col.abs() as usize] = player;
                        }
                        break;
                    }
                }
            }
        }
    }
    // print_board(*board);
}
fn check_for_end() -> bool {
    return true;
}

fn print_board(board: [[char; ROW_SIZE]; COL_SIZE]) {
    let word_array: [char; ROW_SIZE] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    println!(
        "  {:?}",
        word_array
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    for i in 0..ROW_SIZE {
        println!(
            "{} {:?}",
            word_array[i],
            board[i]
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
    println!();
}
