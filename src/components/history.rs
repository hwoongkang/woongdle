use super::guess::Guess;
use super::tile::Tile;
use crate::utils;
use std::fmt;
use termion::clear;

pub struct History {
    guesses: Vec<Guess>,
    chars: [Tile; 26],
    ans: [char; 5],
}

impl History {
    fn print_keyboard(&self) {
        let indices: [usize; 26] = [
            'q' as usize - 'a' as usize,
            'w' as usize - 'a' as usize,
            'e' as usize - 'a' as usize,
            'r' as usize - 'a' as usize,
            't' as usize - 'a' as usize,
            'y' as usize - 'a' as usize,
            'u' as usize - 'a' as usize,
            'i' as usize - 'a' as usize,
            'o' as usize - 'a' as usize,
            'p' as usize - 'a' as usize,
            'a' as usize - 'a' as usize,
            's' as usize - 'a' as usize,
            'd' as usize - 'a' as usize,
            'f' as usize - 'a' as usize,
            'g' as usize - 'a' as usize,
            'h' as usize - 'a' as usize,
            'j' as usize - 'a' as usize,
            'k' as usize - 'a' as usize,
            'l' as usize - 'a' as usize,
            'z' as usize - 'a' as usize,
            'x' as usize - 'a' as usize,
            'c' as usize - 'a' as usize,
            'v' as usize - 'a' as usize,
            'b' as usize - 'a' as usize,
            'n' as usize - 'a' as usize,
            'm' as usize - 'a' as usize,
        ];
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
        println!("{}", self);
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

impl fmt::Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "{}", clear::All);
        writeln!(f, "WORDLE");
        writeln!(f, "");

        writeln!(f, "Guess the WORDLE in 6 tries.");
        writeln!(
            f,
            "Each guess must be a valid 5 letter word. Hit the enter button to submit."
        );
        writeln!(f,"After each guess, the color of the tiles will change to show how close your guess was to the word.");
        writeln!(f, "");
        for tile in self.chars.iter() {
            write!(f, "{}", tile).expect("Failed to write to guess");
        }
        writeln!(f, "");
        writeln!(f, "");
        for guess in self.guesses.iter() {
            write!(f, "{}", guess);
        }
        write!(f, "")
    }
}
