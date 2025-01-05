use regex::Regex;
use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;
use uname::uname;
use users::get_current_username;

pub fn process_grep(input: &str) -> bool {
    let output_raw = Command::new("ps").arg("aux").output().unwrap().stdout;
    let output = String::from_utf8(output_raw)
        .expect("get_count: Failed to convert stdout to string")
        .to_lowercase();
    output.contains(input)
}

pub fn get_uptime() -> String {
    let uptime = System::uptime();
    let hours: u64 = uptime / 3600;
    let minutes: u64 = (uptime % 3600) / 60;
    match (hours, minutes) {
        (0, 0) => "Impossible?".to_string(),
        (0, m) => format!("{m} minutes"),
        (h, 0) => format!("{h} hours"),
        (h, m) => format!("{h} hours {m} minutes"),
    }
}

pub fn get_os_age() -> String {
    let root_metadata = fs::metadata("/").expect("Failed to fetch metadata for root directory");
    let os_creation_time = root_metadata.created().unwrap_or(UNIX_EPOCH);

    let now = SystemTime::now();
    let duration = now
        .duration_since(os_creation_time)
        .expect("System time went backwards");
    let days = duration.as_secs() / 86400;
    let years = days / 365;
    let remaining_days = days % 365;

    match (years, remaining_days) {
        (0, d) => format!("{d} days"),
        (y, 0) => format!("{y} years"),
        (y, d) => format!("{y} years {d} days"),
    }
}

pub fn get_formatting(format: &str) -> String {
    match format {
        "black" => "\x1b[30m",
        "blue" => "\x1b[34m",
        "cyan" => "\x1b[36m",
        "green" => "\x1b[32m",
        "magenta" => "\x1b[35m",
        "red" => "\x1b[31m",
        "yellow" => "\x1b[33m",
        "light_black" => "\x1b[90m",
        "light_blue" => "\x1b[94m",
        "light_cyan" => "\x1b[96m",
        "light_green" => "\x1b[92m",
        "light_magenta" => "\x1b[95m",
        "light_red" => "\x1b[91m",
        "light_yellow" => "\x1b[93m",
        "bold" => "\x1b[1m",
        "reset" => "\x1b[0m",
        _ => "\x1b[32m",
    }
    .to_string()
}

pub fn get_shell() -> String {
    let re = Regex::new(r".*\/(.*)\/").expect("re: Cannot create regex");
    let shell_path = env::var("SHELL").unwrap();
    re.replace_all(&shell_path, "").to_string()
}

pub fn get_kernel_info() -> String {
    let info = uname().unwrap();
    format!("{} {}", info.sysname, info.release)
}

pub fn get_username() -> String {
    let user = get_current_username().unwrap();
    user.to_string_lossy().to_string()
}
