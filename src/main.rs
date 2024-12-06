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
    println!("│{}│ {} │ {}", "        _   _        ".truecolor(152,251,152), " ".truecolor(220,20,60), user);
    println!("│{}│ {} │ {}", "       (.)_(.)       ".truecolor(152,251,152), " ".truecolor(255,127,80), hname);
    println!("│{}│ {} │ {}", r"    _ (   _   ) _    ".truecolor(152,251,152), " ".truecolor(255,255,102), os);
    println!("│{}│ {} │ {}", r"   / \/`-----'\/ \   ".truecolor(152,251,152), " ".truecolor(152,251,152), shell);
    println!("│{}├────┤", r" __\ ( (     ) ) /__ ".truecolor(152,251,152));
    println!("│{}│ {} │ {} | {}", r" )   /\ \._./ /\   ( ".truecolor(152,251,152), " ".truecolor(65,105,225), mem_used / 1024, mem_total / 1024);
    println!("│{}│ {} │ {}", r"  )_/ /|\   /|\ \_(  ".truecolor(152,251,152), "󰅐 ".truecolor(135,206,250), uptime);
    println!("│{}│ {} │ {}", "                     ".truecolor(152,251,152), "󰏖 ".truecolor(238,130,238), packages);
    println!("╰─────────────────────┴────╯");
}