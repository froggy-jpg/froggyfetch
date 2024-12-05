pub mod facts;
pub mod args;
use std::{env, process::{Command, Stdio}};

pub fn fetch_user() -> String {
    let user = Command::new("id").arg("-un").output();
    let user = match user {
        Ok(frog) => String::from_utf8(frog.stdout).unwrap(),
        Err(_) => "Unknown".to_string()
    };
    user.replace('\n', "")
} 

pub fn fetch_hname() -> String {
    let hname = Command::new("uname").arg("-n").output();
    let hname = match hname {
        Ok(frog) => String::from_utf8(frog.stdout).unwrap(),
        Err(_) => "Unknown".to_string()
    };
    hname.replace('\n', "")
}

pub fn fetch_kernel() -> String {
    let kernel = Command::new("uname").arg("-r").output();
    let kernel = match kernel {
        Ok(frog) => String::from_utf8(frog.stdout).unwrap(),
        Err(_) => "Unknown".to_string()
    };
    kernel.replace('\n', "")
}

pub fn fetch_os() -> String {
    let os = Command::new("lsb_release").arg("-sd").output();
    let os = match os {
        Ok(frog) => String::from_utf8(frog.stdout).unwrap(),
        Err(_) => "Unknown".to_string()
    };
    os.replace('\n', "").replace('"', "")
}

pub fn fetch_shell() -> String {
    let shell = "SHELL";
    match env::var(shell) {
        Ok(mut val) => {
            val = val.replace('/', " ");
            val = val.replace('\n', " ");
            val.split(' ').last().unwrap().to_string() 
        }
        Err(_) => "Unknown".to_string(),
    }
}

pub fn fetch_uptime() -> String {
    let uptime = Command::new("uptime").arg("-p").output();
    let uptime = match uptime {
        Ok(frog) => {
            String::from_utf8(frog.stdout)
                .unwrap()
                .replace("hours", "h ")
                .replace("hour", "h ")
                .replace("minutes", "m")
                .replace("minute", "m")
                .replace("days", "d ")
                .replace("day", "d ")
                .replace("up ", "")
        }
        Err(_) => "Unknown".to_string()
    };
    uptime.replace('\n', "").replace(',', "").replace(" ", "")
}

pub fn fetch_used_mem() -> String {
    let mut used1 = Command::new("cat")
        .arg("/proc/meminfo")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let used2 = Command::new("grep")
        .arg("Active:")
        .stdin(Stdio::from(used1.stdout.take().unwrap()))
        .output()
        .unwrap();
    let mem_used = String::from_utf8(used2.stdout).unwrap();
    mem_used.replace('\n', "")
        .replace("Active:          ", "")
        .replace(" kB", "")
}

pub fn fetch_total_mem() -> String {
    let mut total1 = Command::new("cat")
        .arg("/proc/meminfo")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let total2 = Command::new("grep")
        .arg("MemTotal")
        .stdin(Stdio::from(total1.stdout.take().unwrap()))
        .output()
        .unwrap();
    let total3 = String::from_utf8(total2.stdout).unwrap();
    total3.replace('\n', "")
        .replace("MemTotal:       ", "")
        .replace(" kB", "")
}

pub fn fetch_packages() -> String {
    let mut count1 = Command::new("pacman")
        .arg("-Qq")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let count2 = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(count1.stdout.take().unwrap()))
        .output()
        .unwrap();
    let package_count = String::from_utf8(count2.stdout).unwrap();
    package_count.replace('\n', "")
}

