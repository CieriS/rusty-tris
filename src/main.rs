pub mod matrix;
mod game;

use std::io;
use matrix::Matrix as m;
use rand::{random_bool};

fn main() {
    let mut turn:bool = random_bool(1.0 / 2.0);
    println!("its {} turn", if turn { "'X'" } else { "'O'" });
    let mut m = m::default();
    let mut round = 1;
    while round <= 9 {
        println!("round {}", round);
        let row = user_input("row: ");
        let col = user_input("col: ");
        m.set(row, col, turn);
        println!("{}", m);
        if round >= 5 {
            if m.check_win(row, col, turn) {
                println!("{} wins", if turn { "'X'" } else { "'O'" });
                return ()
            };
        };
        turn = !turn;
        round = round + 1;
    }
    println!("That's a draw");
}

fn user_input(msg: &str) -> usize {
    println!("{}", msg);
    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let number = user_input.trim();

        match number.parse::<usize>() {
            Ok(number) => return number,
            Err(_) => {
                println!("invalid input");
                continue;
            },
        }
    }
}
