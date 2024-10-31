use colored::Colorize;

fn main() {
    let hname = froggyfetch::fetch_hname();
    let kernel = froggyfetch::fetch_kernel();
    let user = froggyfetch::fetch_user();
    let os = froggyfetch::fetch_os();
    let shell = froggyfetch::fetch_shell();
    let uptime = froggyfetch::fetch_uptime();
    let mem_used_str = froggyfetch::fetch_used_mem();
    let mem_used = mem_used_str.parse::<i32>().unwrap();
    let mem_total_str = froggyfetch::fetch_total_mem();
    let mem_total = mem_total_str.parse::<i32>().unwrap();
    let packages = froggyfetch::fetch_packages();

    println!("╭─────────────────────┬────────╮");
    println!("│{}│ {} │ {}", "                     ".green(), " user".green(), user);
    println!("│{}│ {} │ {}", "        _   _        ".green(), "󰇅 host".green(), hname);
    println!("│{}│ {} │ {}", "       (.)_(.)       ".green(), " krnl".green(), kernel);
    println!("│{}│ {} │ {}", r"    _ (   _   ) _    ".green(), "  os ".green(), os);
    println!("│{}│ {} │ {}", r"   / \/`-----'\/ \   ".green(), " shll".green(), shell);
    println!("│{}├────────┤", r" __\ ( (     ) ) /__ ".green());
    println!("│{}│ {} │ {} | {}", r" )   /\ \._./ /\   ( ".green(), " memr".green(), mem_used / 1024, mem_total / 1024);
    println!("│{}│ {} │ {}", r"  )_/ /|\   /|\ \_(  ".green(), "󰅐 uptm".green(), uptime);
    println!("│{}│ {} │ {}", "                     ".green(), "󰏖 pkgs".green(), packages);
    println!("╰─────────────────────┴────────╯")
}
