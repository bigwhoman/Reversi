use std::collections::HashMap;
use std::io;
use std::io::Write;

const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;

fn main() {
    game();
}

fn game() {
    let mut board = initialize();
    // board[1][2] = 2;
    // print_board(board);
    // board[2][3] = 5;
    // print_board(board);
    start_game(board);
}

// Initial things -- Num of rows and columns
fn initialize() -> [[char; ROW_SIZE]; COL_SIZE] {
    let mut board: [[char; ROW_SIZE]; COL_SIZE] = [['.'; ROW_SIZE]; COL_SIZE];

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
            board[row][col] = player;
        } else {
            println!("Invalid move");
        }

        print_board(board);
        if player == 'B' {
            player = 'A';
        } else {
            player = 'B';
        }
        if check_for_end() {
            // break;
        }
    }
    println!("Game has ended and {} has won!!!", 12);
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
