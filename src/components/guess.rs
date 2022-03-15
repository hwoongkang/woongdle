use super::tile::Tile;
use std::fmt;
use termion::style;

pub struct Guess {
    pub tiles: [Tile; 5],
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for tile in self.tiles.iter() {
            write!(f, "{}", tile).expect("Failed to write to guess");
        }
        writeln!(f, "{}", style::Reset)
    }
}
impl Guess {
    pub fn is_correct(&self) -> bool {
        self.tiles.iter().all(|tile| {
            if let Tile::Correct(_) = tile {
                true
            } else {
                false
            }
        })
    }
}
