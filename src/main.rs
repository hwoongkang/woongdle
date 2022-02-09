use regex::Regex;
use std::io;
use termion::{color, style};
mod ans;
mod components;
mod utils;
use crate::components::*;

fn main() {
    let ans: [char; 5] = ans::generate();
    let mut history = History::new(ans);

    let five_letter_word = Regex::new(r"^[a-zA-Z]{5}$").unwrap();

    let mut count = 0;
    let flag = loop {
        history.print();
        if count == 6 {
            break false;
        }
        count += 1;
        let local_flag = loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if !five_letter_word.is_match(input) {
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
                break history.add_guess(input);
            }
        };
        if local_flag {
            break true;
        }
    };
    if flag {
        history.print_result();
    } else {
        println!("Please try next time.");
    }
}
