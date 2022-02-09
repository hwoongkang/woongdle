mod assets;
use rand::seq::SliceRandom;

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
