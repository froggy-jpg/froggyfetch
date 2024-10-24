use colored::Colorize;

fn main() {
    let hname = froggyfetch::fetch_hname();
    let kernel = froggyfetch::fetch_kernel();
    let user = froggyfetch::fetch_user();
    let os = froggyfetch::fetch_os();
    let shell = froggyfetch::fetch_shell();
    let uptime = froggyfetch::fetch_uptime();
    let packages = froggyfetch::packages::fetch_packages();

    println!("╭────────╮");
    println!("│  {}  │ {}", "user".green(), user);
    println!("│  {}  │ {}", "host".green(), hname);
    println!("│  {}  │ {}", "krnl".green(), kernel);
    println!("│  {}  │ {}", " os ".green(), os);
    println!("│  {}  │ {}", "shll".green(), shell);
    println!("├────────┤");
    println!("│  {}  │ {}", "uptm".green(), uptime);
    println!("│  {}  │ {}", "pkgs".green(), packages);
    println!("╰────────╯")
}
