use std::fmt;
use termion::{color, style};

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Correct(char),
    Present(char),
    Absent(char),
    Unused(char),
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let fg = color::Fg(color::LightWhite);
        match self {
            Self::Correct(letter) => write!(
                f,
                "{}{}{}{}",
                color::Bg(color::Green),
                fg,
                letter,
                style::Reset
            ),
            Self::Present(letter) => write!(
                f,
                "{}{}{}{}",
                color::Bg(color::Yellow),
                fg,
                letter,
                style::Reset
            ),
            Self::Absent(letter) => write!(
                f,
                "{}{}{}{}",
                color::Bg(color::LightBlack),
                fg,
                letter,
                style::Reset
            ),
            Self::Unused(letter) => write!(
                f,
                "{}{}{}{}",
                color::Bg(color::Black),
                fg,
                letter,
                style::Reset
            ),
        }
    }
}
impl Tile {
    pub fn get_block(&self) -> char {
        match self {
            Self::Correct(_) => '🟩',
            Self::Present(_) => '🟨',
            Self::Absent(_) => '⬛',
            Self::Unused(_) => '⬜',
        }
    }
}
