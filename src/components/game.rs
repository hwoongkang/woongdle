use super::history::History;
use crate::ans;
use std::io;
use termion::{color, style};
pub struct Game {
    history: History,
}

impl Game {
    pub fn new() -> Game {
        let ans = ans::generate();
        let history = History::new(ans);
        Game { history }
    }

    pub fn run(&mut self) -> () {
        let mut count = 0;
        let won = loop {
            println!("{}", self.history);
            if count == 6 {
                break false;
            }
            count += 1;
            if self.turn() {
                break true;
            }
        };
        if won {
            self.history.print_result();
        } else {
            println!("You lost!");
        }
    }

    fn turn(&mut self) -> bool {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if !ans::is_five_letter_word(input) {
                print!("{}", style::Bold);
                print!("{}", color::Fg(color::Red));
                print!("{}", "Invalid input. Please enter a 5 letter word.");
                println!("{}", style::Reset);
                continue;
            } else if !ans::is_valid_word(input) {
                print!(
                    "\"{}{}\" {}is not a valid word.",
                    style::Bold,
                    input,
                    color::Fg(color::Red)
                );
                println!("{}", style::Reset);
                continue;
            } else {
                break self.history.add_guess(input);
            }
        }
    }
}
