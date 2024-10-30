use colored::Colorize;

fn main() {
    let hname = froggyfetch::fetch_hname();
    let kernel = froggyfetch::fetch_kernel();
    let user = froggyfetch::fetch_user();
    let os = froggyfetch::fetch_os();
    let shell = froggyfetch::fetch_shell();
    let uptime = froggyfetch::fetch_uptime();
    let mem_used = froggyfetch::fetch_used_mem();
    let mem_total = froggyfetch::fetch_total_mem();
    let packages = froggyfetch::fetch_packages();

    println!("╭─────────────────────┬────────╮");
    println!("│{}│ {} │ {}", "                     ".green(), " user".green(), user);
    println!("│{}│ {} │ {}", "        _   _        ".green(), "󰇅 host".green(), hname);
    println!("│{}│ {} │ {}", "       (.)_(.)       ".green(), " krnl".green(), kernel);
    println!("│{}│ {} │ {}", r"    _ (   _   ) _    ".green(), "  os ".green(), os);
    println!("│{}│ {} │ {}", r"   / \/`-----'\/ \   ".green(), " shll".green(), shell);
    println!("│{}├────────┤", r" __\ ( (     ) ) /__ ".green());
    println!("│{}│ {} │ {} | {}", r" )   /\ \._./ /\   ( ".green(), "󰅐 mem ".green(), mem_used, mem_total);
    println!("│{}│ {} │ {}", r"  )_/ /|\   /|\ \_(  ".green(), "󰅐 uptm".green(), uptime);
    println!("│{}│ {} │ {}", "                     ".green(), "󰏖 pkgs".green(), packages);
    println!("╰─────────────────────┴────────╯")
}
