use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::io::Write;

const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;

fn main() {
    game();
}

/*
    Start the game.
    Has the potential to change the board size
*/
fn game() {
    let board = initialize();
    start_game(board);
}

// Initial board -- Num of rows and columns
fn initialize() -> [[char; ROW_SIZE]; COL_SIZE] {
    // initialize the board to all '.'
    let mut board: [[char; ROW_SIZE]; COL_SIZE] = [['.'; ROW_SIZE]; COL_SIZE];
    board[3][3] = 'W';
    board[4][4] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';

    return board;
}

/*
    The core of our game
*/
fn start_game(mut board: [[char; ROW_SIZE]; COL_SIZE]) {
    let mut player = 'B';
    /*
            Create a hashmap that maps a..h to 0..9
            This is for the sake of decrypting users input to actual -
                - locations in the board
    */
    let map: HashMap<char, usize> = ('a'..='h').zip(0..9).collect();
    let mut game_end: i32 = 0;
    print_board(board);
    loop {
        loop {
            // check if a player can have a valid move
            if cant_place(&mut board, player) {
                println!("{} player has no valid move.", player);
                game_end += 1;
                // This is to avoid having an endless loop of players with no moves
                if game_end >= 2 {
                    let mut w_pieces = 0;
                    let mut b_pieces = 0;

                    // Dumb way to caclulate the points. It would have been better to take -
                    // - record in the midst of the game to avoid this loop
                    for i in 0..ROW_SIZE {
                        for j in 0..COL_SIZE {
                            if board[i][j] == 'W' {
                                w_pieces += 1;
                            } else if board[i][j] == 'B' {
                                b_pieces += 1;
                            }
                        }
                    }
                    let diff = w_pieces - b_pieces;
                    if diff < 0 {
                        print!("Black wins by {} points!", -diff);
                    } else if diff > 0 {
                        print!("White wins by {} points!", diff);
                    } else {
                        print!("Draw!")
                    }

                    return;
                }
                // no moves = change player and continue the game :D
                player = change_player(player);
                continue;
            }

            // if a player has a valid move then we should avoid ending the game
            game_end = max(game_end - 1, 0);

            let mut input_line: String = Default::default();

            print!("Enter move for colour {} (RowCol): ", player);

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_line).expect("Wrong input");

            let row_move: char = input_line.trim().chars().nth(0).unwrap();
            let col_move: char = input_line.trim().chars().nth(1).unwrap();

            // This is to map players move to numbers and then play the move
            if let (Some(&row), Some(&col)) = (map.get(&row_move), map.get(&col_move)) {
                if board[row][col] == '.' {
                    // set the move and then see if that move is valid or not
                    board[row][col] = player;
                    // if the move was valid then break out of the loop to continue -
                    // - your game; Otherwise just do another move
                    let valid_move: bool = do_move(&mut board, row, col, player, Some(false));
                    if valid_move {
                        break;
                    }
                    board[row][col] = '.';
                }
            }
            println!("Invalid move. Try again.");
            print_board(board);
        }

        print_board(board);
        player = change_player(player);
    }
    // println!("Game has ended and {} has won!!!", 12);
}

/*
    Simple function which changes the players
*/
fn change_player(player: char) -> char {
    let new_player: char;
    if player == 'B' {
        new_player = 'W';
    } else {
        new_player = 'B';
    }
    return new_player;
}

/*
    The logic core of the game. Pretty easy I might say
    The return value is if the move was valid or not
*/
fn do_move(
    board: &mut [[char; ROW_SIZE]; COL_SIZE],
    row: usize,
    col: usize,
    player: char,
    just_check: Option<bool>,
) -> bool {
    let mut valid_move: bool = false;

    for i in [-1_i32, 0, 1] {
        for j in [-1_i32, 0, 1] {
            let mut temp_row: i32 = row as i32;
            let mut temp_col: i32 = col as i32;
            if i == 0 && j == 0 {
                continue;
            } else {
                while (0_i32..ROW_SIZE as i32).contains(&temp_row)
                    && (0_i32..COL_SIZE as i32).contains(&temp_col)
                    && ['W', 'B']
                        .iter()
                        .any(|&x| x == board[(temp_row).abs() as usize][(temp_col).abs() as usize])
                {
                    temp_row += i;
                    temp_col += j;
                    // Just check all the 8 directions around where player wants to place his move
                    // See if in any of 8 directions, the opponents blocks would be sandwiched :D
                    if (0_i32..ROW_SIZE as i32).contains(&temp_row)
                        && (0_i32..COL_SIZE as i32).contains(&temp_col)
                        && board[temp_row.abs() as usize][temp_col.abs() as usize] == player
                    {
                        while temp_row != row as i32 || temp_col != col as i32 {
                            temp_col -= j;
                            temp_row -= i;
                            if board[temp_row.abs() as usize][temp_col.abs() as usize] != player {
                                valid_move = true;
                                // The just check is to find if the move is valid or not which is used in the "cant place" -
                                // - function and not to actually do the move
                                if just_check.unwrap_or(false) && valid_move {
                                    return true;
                                }
                                board[temp_row.abs() as usize][temp_col.abs() as usize] = player;
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    // print_board(*board);
    return valid_move;
}

/*
    Checks if the game has ended and if it is,
    Shows the final stats of the game
    Pretty simple implementation whcih checks every '.' in the board -
    - and then sees if that is a valid move.
    Pretty sure that there would be an optimized way of doing this but I did not care about optimization that much :(
*/
fn cant_place(board: &mut [[char; ROW_SIZE]; COL_SIZE], player: char) -> bool {
    for i in 0..ROW_SIZE {
        for j in 0..COL_SIZE {
            if board[i][j] == '.' {
                board[i][j] = player;

                if do_move(board, i, j, player, Some(true)) {
                    board[i][j] = '.';
                    return false;
                }
                board[i][j] = '.';
            }
        }
    }

    return true;
}

/*
    Simple function to print your board
*/
fn print_board(board: [[char; ROW_SIZE]; COL_SIZE]) {
    let word_array: [char; ROW_SIZE] = ('a'..='h').collect::<Vec<_>>().try_into().unwrap();
    println!(
        "  {}",
        word_array
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    for i in 0..ROW_SIZE {
        println!(
            "{} {}",
            word_array[i],
            board[i]
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}
