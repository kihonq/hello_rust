use chrono::prelude::*;
use std::io::stdin;

enum State {
    Locked,
    Unlocked,
    Failed,
}

fn now_str () -> String {
    Local::now().format("%H%M").to_string()
}

pub fn main() {
    println!("\n/============= Combination Lock =============/\n");
    println!("Please enter your combination 🔐:");
    
    let mut code = now_str();

    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("Combination failed ! ❌");
                entry.clear();
                state = State::Locked;
                code = now_str();
                continue;
            }

            State::Unlocked => {
                println!("Combination success ! 🔓");
                break;
            }
        }
    }
}
