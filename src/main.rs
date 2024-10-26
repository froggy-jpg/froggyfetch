use colored::Colorize;

fn main() {
    let hname = froggyfetch::fetch_hname();
    let kernel = froggyfetch::fetch_kernel();
    let user = froggyfetch::fetch_user();
    let os = froggyfetch::fetch_os();
    let shell = froggyfetch::fetch_shell();
    let uptime = froggyfetch::fetch_uptime();
    let packages = froggyfetch::packages::fetch_packages();

    println!("╭─────────────────────┬────────╮");
    println!("│{}│  {}  │ {}", "        _   _        ".green(), "user".green(), user);
    println!("│{}│  {}  │ {}", "       (.)_(.)       ".green(), "host".green(), hname);
    println!("│{}│  {}  │ {}", "    _ (   _   ) _    ".green(), "krnl".green(), kernel);
    println!("│{}│  {}  │ {}", r"   / \/`-----'\/ \   ".green(), " os ".green(), os);
    println!("│{}│  {}  │ {}", r" __\ ( (     ) ) /__ ".green(), "shll".green(), shell);
    println!("│{}├────────┤", r" )   /\ \._./ /\   ( ".green());
    println!("│{}│  {}  │ {}", r"  )_/ /|\   /|\ \_(  ".green(), "uptm".green(), uptime);
    println!("│{}│  {}  │ {}", "                     ".green(), "pkgs".green(), packages);
    println!("╰─────────────────────┴────────╯")
}
