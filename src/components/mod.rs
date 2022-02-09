use crate::utils;
use termion::{clear, color, style};

type Bg = color::Bg<color::Rgb>;
type Fg = color::Fg<color::Rgb>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    Correct,
    Present,
    Absent,
    Unused,
}

impl State {
    pub fn get_color(&self) -> (Bg, Fg) {
        let bg = match self {
            State::Correct => color::Bg(color::Rgb(83, 141, 78)),
            State::Present => color::Bg(color::Rgb(181, 159, 59)),
            State::Absent => color::Bg(color::Rgb(58, 58, 60)),
            State::Unused => color::Bg(color::Rgb(129, 131, 132)),
        };
        (bg, color::Fg(color::Rgb(215, 218, 220)))
    }

    pub fn get_block(&self) -> char {
        match self {
            State::Correct => '🟩',
            State::Present => '🟨',
            State::Absent => '⬛',
            State::Unused => '⬜',
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    letter: char,
    guess: State,
}

impl Tile {
    pub fn print(&self) {
        let (bg, fg) = self.guess.get_color();
        print!("{}{}{}", bg, fg, self.letter);
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

    fn from(word: &str, ans: &[char; 5]) -> Self {
        let mut tiles = [Tile {
            letter: ' ',
            guess: State::Absent,
        }; 5];
        word.to_lowercase()
            .chars()
            .zip(ans)
            .map(|(letter, truth)| {
                let guess = if letter == *truth {
                    State::Correct
                } else if ans.contains(&letter) {
                    State::Present
                } else {
                    State::Absent
                };

                Tile { letter, guess }
            })
            .enumerate()
            .for_each(|(i, tile)| {
                tiles[i] = tile;
            });
        Guess { tiles }
    }

    fn is_correct(&self) -> bool {
        self.tiles.iter().all(|tile| tile.guess == State::Correct)
    }
}

pub struct History {
    guesses: Vec<Guess>,
    chars: [State; 26],
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
        self.chars
            .iter()
            .enumerate()
            .map(|(ind, state)| Tile {
                letter: utils::u8_to_char(ind as u8),
                guess: *state,
            })
            .for_each(|tile| tile.print());
        println!("{}", style::Reset);
        println!("");

        for guess in self.guesses.iter() {
            guess.print();
        }
    }

    pub fn new(ans: [char; 5]) -> Self {
        History {
            ans,
            guesses: Vec::new(),
            chars: [State::Unused; 26],
        }
    }

    pub fn add_guess(&mut self, word: &str) -> bool {
        let mut tiles = [Tile {
            letter: ' ',
            guess: State::Absent,
        }; 5];
        word.to_lowercase()
            .chars()
            .zip(&self.ans)
            .map(|(letter, truth)| {
                let state = if letter == *truth {
                    State::Correct
                } else if self.ans.contains(&letter) {
                    State::Present
                } else {
                    State::Absent
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
        let guess = Guess { tiles };
        let flag = guess.is_correct();
        self.guesses.push(Guess::from(word, &self.ans));
        flag
    }

    pub fn print_result(&self) {
        self.print();
        println!("");
        println!("Your wordle:");
        for guess in self.guesses.iter() {
            for tile in guess.tiles.iter() {
                print!("{}", tile.guess.get_block());
            }
            println!("");
        }
    }
}
