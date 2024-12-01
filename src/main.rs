use colored::Colorize;

fn main() {
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

    println!("╭─────────────────────┬────╮");
    println!("│{}│ {} │ {}", "        _   _        ".green(), " ".green(), user);
    println!("│{}│ {} │ {}", "       (.)_(.)       ".green(), " ".green(), hname);
    println!("│{}│ {} │ {}", r"    _ (   _   ) _    ".green(), " ".green(), os);
    println!("│{}│ {} │ {}", r"   / \/`-----'\/ \   ".green(), " ".green(), shell);
    println!("│{}├────┤", r" __\ ( (     ) ) /__ ".green());
    println!("│{}│ {} │ {} | {}", r" )   /\ \._./ /\   ( ".green(), " ".green(), mem_used / 1024, mem_total / 1024);
    println!("│{}│ {} │ {}", r"  )_/ /|\   /|\ \_(  ".green(), "󰅐 ".green(), uptime);
    println!("│{}│ {} │ {}", "                     ".green(), "󰏖 ".green(), packages);
    println!("╰─────────────────────┴────╯");

}