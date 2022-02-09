use super::ans;
use crate::utils;
use std::io;
use termion::{clear, color, style};

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Correct(char),
    Present(char),
    Absent(char),
    Unused(char),
}

impl Tile {
    pub fn print(&self) {
        let fg = color::Fg(color::LightWhite);
        match self {
            Self::Correct(letter) => print!("{}{}{}", color::Bg(color::Green), fg, letter),
            Self::Present(letter) => print!("{}{}{}", color::Bg(color::Yellow), fg, letter),
            Self::Absent(letter) => print!("{}{}{}", color::Bg(color::LightBlack), fg, letter),
            Self::Unused(letter) => print!("{}{}{}", color::Bg(color::Black), fg, letter),
        }
    }
    pub fn get_block(&self) -> char {
        match self {
            Self::Correct(_) => '🟩',
            Self::Present(_) => '🟨',
            Self::Absent(_) => '⬛',
            Self::Unused(_) => '⬜',
        }
    }
}

pub struct Guess {
    tiles: [Tile; 5],
}

impl Guess {
    fn print(&self) {
        self.tiles.iter().for_each(|tile| tile.print());
        println!("{}", style::Reset);
    }

    fn is_correct(&self) -> bool {
        self.tiles.iter().all(|tile| {
            if let Tile::Correct(_) = tile {
                true
            } else {
                false
            }
        })
    }
}

pub struct History {
    guesses: Vec<Guess>,
    chars: [Tile; 26],
    ans: [char; 5],
}

impl History {
    pub fn print(&self) {
        println!("{}", clear::All);
        println!("WORDLE");
        println!("");

        println!("Guess the WORDLE in 6 tries.");
        println!("Each guess must be a valid 5 letter word. Hit the enter button to submit.");
        println!("After each guess, the color of the tiles will change to show how close your guess was to the word.");
        println!("");
        self.chars.iter().for_each(|tile| tile.print());
        println!("{}", style::Reset);
        println!("");

        for guess in self.guesses.iter() {
            guess.print();
        }
    }

    pub fn new(ans: [char; 5]) -> Self {
        let mut chars = [Tile::Unused(' '); 26];
        for ind in 0..26 {
            chars[ind] = Tile::Unused(utils::u8_to_char(ind as u8));
        }
        History {
            ans,
            chars,
            guesses: Vec::new(),
        }
    }

    pub fn add_guess(&mut self, word: &str) -> bool {
        let mut tiles = [Tile::Absent(' '); 5];
        word.to_lowercase()
            .chars()
            .zip(&self.ans)
            .map(|(letter, truth)| {
                let tile = if letter == *truth {
                    Tile::Correct(letter)
                } else if self.ans.contains(&letter) {
                    Tile::Present(letter)
                } else {
                    Tile::Absent(letter)
                };
                self.chars[utils::char_to_usize(letter)] = tile;
                tile
            })
            .enumerate()
            .for_each(|(i, tile)| {
                tiles[i] = tile;
            });
        let guess = Guess { tiles };
        let flag = guess.is_correct();
        self.guesses.push(guess);
        flag
    }

    pub fn print_result(&self) {
        self.print();
        println!("");
        println!("Your wordle:");
        for guess in self.guesses.iter() {
            for tile in guess.tiles.iter() {
                print!("{}", tile.get_block());
            }
            println!("");
        }
    }
}

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
            self.history.print();
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
