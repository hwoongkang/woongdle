mod assets;
use rand::seq::SliceRandom;
use regex::Regex;

pub fn generate() -> [char; 5] {
    let mut ans = ['a'; 5];
    assets::CANDIDATES
        .choose(&mut rand::thread_rng())
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(ind, ch)| {
            ans[ind] = ch;
        });
    ans
}

pub fn is_valid_word(word: &str) -> bool {
    assets::CANDIDATES.contains(&word) || assets::VALID_WORDS.contains(&word)
}

pub fn is_five_letter_word(word: &str) -> bool {
    let five_letter_word = Regex::new(r"^[a-zA-Z]{5}$").unwrap();
    five_letter_word.is_match(word)
}
