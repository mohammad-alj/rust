use std::convert::TryInto;
use std::io;

struct Game {
    board: [[char; 3]; 3],
    current_player: char,
    avaliable_spaces: u8,
}

enum RoundResult {
    X,
    O,
    TIE,
    NONE,
}

fn main() {
    let mut g = Game {
        board: [[' '; 3]; 3],
        current_player: 'X',
        avaliable_spaces: 9,
    };

    println!("Welcome to TIC-TAC-TOE");

    print_board(&g);
    loop {
        println!("{} player's turn!", g.current_player);
        let cor = prompt_space();
        if !place_symbol(cor, &mut g) {
            continue;
        }

        print_board(&g);

        match check_round_results(&mut g) {
            RoundResult::X => {
                println!("Player X won!");
                break;
            }
            RoundResult::O => {
                println!("Player O won!");
                break;
            }
            RoundResult::TIE => {
                println!("It is a tie!");
                break;
            }
            RoundResult::NONE => {
                continue;
            }
        }
    }
}

fn prompt_space() -> (usize, usize) {
    let x: usize = (prompt("Enter row", Some(|x| 1 <= x && x <= 3)) - 1)
        .try_into()
        .expect("Failed to convert row coordanite.");
    let y: usize = (prompt("Enter column", Some(|x| 1 <= x && x <= 3)) - 1)
        .try_into()
        .expect("Failed to convert column coordanite.");
    (x, y)
}

fn prompt(message: &str, validator_func: Option<fn(i32) -> bool>) -> i32 {
    println!("{}", message);
    loop {
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line!");
        let x: i32 = match x.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match validator_func {
            Some(func) => {
                if !func(x) {
                    continue;
                }
            }
            None => {}
        }
        return x;
    }
}

fn print_board(g: &Game) {
    println!("  1   2   3");
    println!("    |   |   ");
    println!(
        "1 {} | {} | {} ",
        g.board[0][0], g.board[0][1], g.board[0][2]
    );
    println!(" ___|___|___");
    println!("    |   |   ");
    println!(
        "2 {} | {} | {} ",
        g.board[1][0], g.board[1][1], g.board[1][2]
    );
    println!(" ___|___|___");
    println!("    |   |   ");
    println!(
        "3 {} | {} | {} ",
        g.board[2][0], g.board[2][1], g.board[2][2]
    );
    println!("    |   |   ");
}

fn place_symbol(coordinates_pair: (usize, usize), g: &mut Game) -> bool {
    let space = &mut g.board[coordinates_pair.0][coordinates_pair.1];
    if *space == ' ' {
        *space = g.current_player;
        g.current_player = if g.current_player == 'X' { 'O' } else { 'X' };
        g.avaliable_spaces -= 1;
        return true;
    }
    return false;
}

fn check_round_results(g: &mut Game) -> RoundResult {
    let b = &g.board;

    for row in b {
        if row[0] != ' ' && row[0] == row[1] && row[1] == row[2] {
            return if row[0] == 'X' {
                RoundResult::X
            } else {
                RoundResult::O
            };
        }
    }

    for i in 0..3 {
        if b[0][i] != ' ' && b[0][i] == b[1][i] && b[1][i] == b[2][i] {
            return if b[i][0] == 'X' {
                RoundResult::X
            } else {
                RoundResult::O
            };
        }
    }

    if (b[0][0] != ' ' && b[0][0] == b[1][1] && b[1][1] == b[2][2])
        || (b[0][2] != ' ' && b[0][2] == b[1][1] && b[1][1] == b[2][0])
    {
        return if b[1][1] == 'X' {
            RoundResult::X
        } else {
            RoundResult::O
        };
    }

    return if g.avaliable_spaces == 0 {
        RoundResult::TIE
    } else {
        RoundResult::NONE
    };
}
