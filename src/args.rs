use std::env;
use colored::Colorize;
use crate::facts;

// a bit janky, but checks if the user has set a flag by using an if statement
pub fn fetch_options() -> bool {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        help_msg();
        return true;
    }

    if args.contains(&String::from("-f")) || args.contains(&String::from("--fact")) {
        frog_fact();
        return true;
    }
    false
}

// prints help message
pub fn help_msg() {
    println!("{}", "OPTIONS".truecolor(152, 251, 152).bold());
    println!("{} show a random frog fact", "  -f, --fact ".bold());
    println!("{} show this message", "  -h, --help ".bold());
}

// imports facts from facts.rs
fn frog_fact() {
    let _frog_fact = facts::frog_fact();
}