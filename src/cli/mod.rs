use std::io::{self, Write};

pub enum PromptResult {
    Yes,
    No,
    Default,
}

pub fn prompt(message: &str, default_option: Option<&str>) -> PromptResult {
    loop {
        if let Some(default_option) = default_option {
            print!("{}(yes/no/[{}]) ", message, default_option);
        } else {
            print!("{}([yes]/no) ", message);
        }
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match (input.trim()) {
            "yes" | "y" => return PromptResult::Yes,
            "no" | "n" => return PromptResult::No,
            "" => return PromptResult::Default,
            _ => println!("Invalid input"),
        }
    }
}
