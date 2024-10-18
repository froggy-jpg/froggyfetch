fn main() {
    let hname = froggyfetch::fetch_hname();
    let kernel = froggyfetch::fetch_kernel();
    let user = froggyfetch::fetch_user();
    let os = froggyfetch::fetch_os();
    let shell = froggyfetch::fetch_shell();
    let uptime = froggyfetch::fetch_uptime();
    let packages = froggyfetch::packages::fetch_packages();

    println!("user: {user}");
    println!("host: {hname}");
    println!("kernel: {kernel}");
    println!("os: {os}");
    println!("shell: {shell}");
    println!("uptime: {uptime}");
    println!("packages: {packages}")
}
