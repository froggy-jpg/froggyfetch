use std::env;
use colored::Colorize;
use crate::facts;

pub fn fetch_options() -> bool {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-h")) {
       help_msg();
       return true;
    }

    if args.contains(&String::from("-f")) {
        frog_fact();
        return true;
     } false
}

pub fn help_msg() {
    println!("{}", "OPTIONS".green().bold());
    println!("  -f, --fact  show a random frog fact");
    println!("  -h, --help  show this message");
}

fn frog_fact() {
    let frog_fact = facts::frog_fact();
}