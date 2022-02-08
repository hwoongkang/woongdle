use regex::Regex;
use std::io;
use termion::{clear, color, style};
mod enums;
mod utils;

#[derive(Copy, Clone)]
struct Tile {
    letter: char,
    guess: enums::State,
}

impl Tile {
    fn print(&self) {
        let (bg, fg) = self.guess.get_color();
        print!("{}{}{}", bg, fg, self.letter);
    }
}

struct Guess {
    tiles: [Tile; 5],
}

impl Guess {
    fn print(&self) {
        self.tiles.iter().for_each(|tile| tile.print());
        println!("{}", style::Reset);
    }

    fn from(word: &str, ans: &[char; 5]) -> Self {
        let mut tiles = [Tile {
            letter: ' ',
            guess: enums::State::Absent,
        }; 5];
        word.to_lowercase()
            .chars()
            .zip(ans)
            .map(|(letter, truth)| {
                let guess = if letter == *truth {
                    enums::State::Correct
                } else if ans.contains(&letter) {
                    enums::State::Present
                } else {
                    enums::State::Absent
                };

                Tile { letter, guess }
            })
            .enumerate()
            .for_each(|(i, tile)| {
                tiles[i] = tile;
            });
        Guess { tiles }
    }
}

struct History {
    guesses: Vec<Guess>,
    chars: [enums::State; 26],
    ans: [char; 5],
}

impl History {
    fn print(&self) {
        println!("WORDLE");
        println!("");
        if self.guesses.len() == 0 {
            println!("Guess the WORDLE in 6 tries.");
            println!("Each guess must be a valid 5 letter word. Hit the enter button to submit.");
            println!("After each guess, the color of the tiles will change to show how close your guess was to the word.");
        } else {
            self.chars
                .iter()
                .enumerate()
                .map(|(ind, state)| Tile {
                    letter: utils::u8_to_char(ind as u8),
                    guess: *state,
                })
                .for_each(|tile| tile.print());
            println!("{}", style::Reset);
        }

        for guess in self.guesses.iter() {
            guess.print();
        }
    }

    fn new(ans: [char; 5]) -> Self {
        History {
            ans,
            guesses: Vec::new(),
            chars: [enums::State::Unused; 26],
        }
    }

    fn add_guess(&mut self, word: &str) {
        let mut tiles = [Tile {
            letter: ' ',
            guess: enums::State::Absent,
        }; 5];
        word.to_lowercase()
            .chars()
            .zip(&self.ans)
            .map(|(letter, truth)| {
                let state = if letter == *truth {
                    enums::State::Correct
                } else if self.ans.contains(&letter) {
                    enums::State::Present
                } else {
                    enums::State::Absent
                };
                self.chars[utils::char_to_usize(letter)] = state;
                Tile {
                    letter,
                    guess: state,
                }
            })
            .enumerate()
            .for_each(|(i, tile)| {
                tiles[i] = tile;
            });
        self.guesses.push(Guess::from(word, &self.ans));
    }
}

fn main() {
    let ans: [char; 5] = ['f', 'r', 'a', 'm', 'e'];
    let mut history = History::new(ans);

    let five_letter_word = Regex::new(r"^[a-zA-Z]{5}$").unwrap();

    for _ in 0..5 {
        println!("{}", clear::All);
        history.print();
        loop {
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
            } else {
                break history.add_guess(input);
            }
        }
    }
}
