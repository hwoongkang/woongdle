mod ans;
mod cli;
mod components;
mod utils;
use components::*;

fn main() {
    let hard_mode = loop {
        match cli::prompt("Hard mode?", Some("explain")) {
            cli::PromptResult::Yes => break true,
            cli::PromptResult::No => break false,
            cli::PromptResult::Default => {
                println!("Any revealed hints must be used in subsequent guesses in hard mode.")
            }
        }
    };
    let mut game = Game::new();
    game.run();
}
