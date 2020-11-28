use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result;
use rand::Rng;

enum Condition {
    HOT,
    OK,
    COLD,
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Condition::HOT => {
                write!(f, "HOT 🥵")
            }
            Condition::OK => {
                write!(f, "OK 😌")
            }
            Condition::COLD => {
                write!(f, "COLD 🥶")
            }
        }
    }
}

pub fn main() {
    println!("\n/============= If Statement =============/\n");

    let mut rng = rand::thread_rng();
    let temp: f32 = rng.gen_range(-23.9, 56.7);
    let mut cond: Condition = Condition::OK;

    if temp > 30.0 {
        cond = Condition::HOT;
    } else if temp < 10.0 {
        cond = Condition::COLD;
    }

    println!("temp is {}, and it's {}", format!("{:.2}", temp), cond)
}
