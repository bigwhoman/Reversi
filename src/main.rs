use std::io;
struct User {
    
}

fn main() {
    game();
}

fn game(){
    let aa  = initialize();
    println!("{:?}",aa);
    start_game();
}


// Initial things -- Num of rows and columns
fn initialize() -> (Vec<Vec<i32>>, Vec<User>){
    let _a:User;
    let mut input_line:String = Default::default();
    let size;

    println!("Input the size of your tic-tac-toe");

    io::stdin().read_line(&mut input_line)
                .expect("Wrong input");

    size = input_line.trim()
                    .parse()
                    .expect("Input not an integer");

    assert!(size > 2, "Size must be more than 2 !!");

    let mut array = vec![vec![0; size]; size];
    return (array, vec![User;2]);
}

fn start_game(){
    loop{
        // pass 
        // ...

        if check_for_end() {
            break;
        }
    }
    println!("Game has ended and {} has won!!!",12);
}

fn check_for_end() -> bool{
    return true;
}




