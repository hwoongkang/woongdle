// Get alphabet from index
// 0 is 'a'
// 1 is 'b'
pub fn u8_to_char(ind: u8) -> char {
    ('a' as u8 + ind) as char
}

pub fn char_to_usize(ch: char) -> usize {
    ch as usize - 'a' as usize
}
