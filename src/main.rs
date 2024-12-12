// import the colored library for... uhhh... colors :frog:
use colored::Colorize;

fn main() {
// all the info imports from lib.rs
    let user = froggyfetch::fetch_user();
    let hname = froggyfetch::fetch_hname();
//  let kernel = froggyfetch::fetch_kernel();
    let os = froggyfetch::fetch_os();
    let shell = froggyfetch::fetch_shell();
    let mem_used_str = froggyfetch::fetch_used_mem();
    let mem_used = mem_used_str.parse::<i32>().unwrap();
    let mem_total_str = froggyfetch::fetch_total_mem();
    let mem_total = mem_total_str.parse::<i32>().unwrap();
    let uptime = froggyfetch::fetch_uptime();
    let packages = froggyfetch::fetch_packages();

// if cli arguments are run dont print the frog below :frog:
    if froggyfetch::args::fetch_options() {
        return;
    }

// print the info and frog ascii, the first {} is used to print the frog ascii so that i can use colored,
// then the nerdfont characters for system info and info itself 
    println!("╭─────────────────────┬────╮");
    println!("│{}│ {} │ {}", "        _   _        ".green(), " ".red(), user);
    println!("│{}│ {} │ {}", "       (.)_(.)       ".green(), " ".yellow(), hname);
    println!("│{}│ {} │ {}", r"    _ (   _   ) _    ".green(), " ".green(), os);
    println!("│{}│ {} │ {}", r"   / \/`-----'\/ \   ".green(), " ".blue(), shell);
    println!("│{}├────┤", r" __\ ( (     ) ) /__ ".green());
    println!("│{}│ {} │ {} | {}", r" )   /\ \._./ /\   ( ".green(), " ".cyan(), mem_used / 1024, mem_total / 1024);
    println!("│{}│ {} │ {}", r"  )_/ /|\   /|\ \_(  ".green(), "󰅐 ".purple(), uptime);
    println!("│{}│ {} │ {}", "                     ".green(), "󰏖 ".green(), packages);
    println!("╰─────────────────────┴────╯");
}