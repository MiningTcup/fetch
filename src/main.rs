use std::{env, fs, time::Duration, collections::HashMap};
use local_ip_address::local_ip;

fn main() {
    match env::var("USER") {
        Ok(user) => print!("\x1b[31m\x1b[1m      _____       \x1b[32m{}", user),
        Err(e) => eprint!("\x1b[31m\x1b[1m      _____        \x1b[0mCouldn't read $USER: {}", e),
    }
    match fs::read_to_string("/etc/hostname") {
        Ok(hostname) => print!("@{}", hostname),
        Err(e) => eprintln!("@Couldn't read /etc/hostname: {}", e),
    }

    match fs::read_to_string("/etc/os-release") {
        Ok(os_release) => {
            for line in os_release.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    let pretty_name = value.trim_matches('"');
                    println!("\x1b[31m     /  __ \\      \x1b[31mos     \x1b[0m{}", pretty_name);
                }
            }
        },
        Err(e) => eprintln!("\x1b[31m     /  __ \\      \x1b[31mos     \x1b[0mCouldn't read /etc/os-release: {}", e),
    }


    match local_ip() {
        Ok(ip) => println!("\x1b[31m\x1b[1m    |  /    |     \x1b[31mlocal  \x1b[0m{}", ip),
        Err(e) => eprintln!("\x1b[31m\x1b[1m    |  /    |     \x1b[31mlocal  \x1b[0mCouldn't get local IP address: {}", e),
    }

    match fs::read_to_string("/proc/sys/kernel/ostype") {
        Ok(ostype) => print!("\x1b[31m\x1b[1m    |  \\___-      \x1b[31mkernel \x1b[0m{} ", ostype.trim_end()),
        Err(e) => eprint!("\x1b[31m\x1b[1m    |  \\___-      \x1b[31mkernel \x1b[0mCouldn't read /proc/sys/kernel/ostype: {}", e),
    }
    match fs::read_to_string("/proc/sys/kernel/osrelease") {
        Ok(osrelease) => print!("{}", osrelease),
        Err(e) => println!("Couldn't read /proc/sys/kernel/osrelease: {}", e),
    }

    match fs::read_to_string("/proc/uptime") {
        Ok(raw_uptime) => {
            if let Some(first_part) = raw_uptime.split_whitespace().next() {
                if let Ok(seconds) = first_part.split('.').next().unwrap_or("0").parse::<u64>() {
                    let duration = Duration::from_secs(seconds);
                    let days = duration.as_secs() / 86400;
                    let hours = (duration.as_secs() / 3600) % 24;
                    let minutes = (duration.as_secs() / 60) % 60;
                    let seconds = duration.as_secs() % 60;

                    let uptime_string = format!(
                        "{}{}{}{}s",
                        if days > 0 { format!("{}d ", days) } else { String::new() },
                        if hours > 0 { format!("{}h ", hours) } else { String::new() },
                        if minutes > 0 { format!("{}m ", minutes) } else { String::new() },
                        seconds
                    );

                    println!("\x1b[31m\x1b[1m    -_            \x1b[31muptime \x1b[0m{}", uptime_string.trim());
                }
            }
        }
        Err(e) => eprintln!("\x1b[31m\x1b[1m    -_            \x1b[31muptime \x1b[0mCouldn't read /proc/uptime: {}", e),
    }

    let mut cores: f32 = 1.0;
    match fs::read_to_string("/sys/devices/system/cpu/present") {
        Ok(cpu_present) => cores = cpu_present[2..].trim().parse::<f32>().unwrap_or(0.0) + 1.0,
        Err(e) => print!("Couldn't read /sys/devices/system/cpu/present: {}", e),
    }
    match fs::read_to_string("/proc/loadavg") {
        Ok(loadavg) => print!("\x1b[31m\x1b[1m      --_         \x1b[31mcpu    \x1b[0m{}%\n", (loadavg.split_whitespace().next().unwrap().parse::<f32>().expect("Couldn't parse /proc/loadavg") / (cores + 1.0) * 100.0).floor()),
        Err(e) => eprintln!("\x1b[31m\x1b[1m      --_         \x1b[31mcpu  \x1b[0mCouldn't read /proc/loadavg: {}", e),
    }
    
    match fs::read_to_string("/proc/meminfo") {
        Ok(meminfo) => {
            let mut mem_map = HashMap::new();

            for line in meminfo.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().to_string();
                    let value = value.trim().split_whitespace().next().unwrap_or("0");
                    if let Ok(parsed_value) = value.parse::<u64>() {
                        mem_map.insert(key, parsed_value);
                    }
                }
            }

            let sizes = ["KB", "MB", "GB", "TB"];

            let used_memory = (*mem_map.get("MemTotal").unwrap_or(&0)
            - *mem_map.get("MemFree").unwrap_or(&0)
            - *mem_map.get("Buffers").unwrap_or(&0)
            - *mem_map.get("Cached").unwrap_or(&0)
            - *mem_map.get("SReclaimable").unwrap_or(&0)) as f64;

            let mut size = used_memory;
            let mut unit = "KB";

            for &next_unit in &sizes {
                if size < 1024.0 {
                    unit = next_unit;
                    break;
                }
                size /= 1024.0;
            }

            print!("\x1b[31m\x1b[1m                  \x1b[31mmemory \x1b[0m{:.1} {}", size, unit);
            
            let mut size = *mem_map.get("MemTotal").unwrap_or(&0) as f64;
            let mut unit = "KB";

            for &next_unit in &sizes {
                if size < 1024.0 {
                    unit = next_unit;
                    break;
                }
                size /= 1024.0;
            }

            println!(" / {:.1} {}\n", size, unit);
        }
        Err(e) => print!("\x1b[31m\x1b[1m                  \x1b[31mmemory \x1b[0mCouldn't read /proc/meminfo: {}", e),
    }
}


/*
  _____    ted@tedserver
 /  __ \   os     Debian GNU/Linux 12 (bookworm)
|  /    |  host   SER
|  \___-   kernel 6.1.0-30-amd64
-_         uptime 8d 1h 27m
  --_      pkgs   1429
           memory 3243M / 12889M
*/