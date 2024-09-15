use sysinfo::System;
use users::get_current_username;

struct SystemInfo {
    os: String,
    kernel_version: String,
    hostname: String,
    username: String,
}

impl SystemInfo {
    fn new() -> Self {
        let os = System::name().unwrap_or("Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or("Unknown".to_string());
        let hostname = System::host_name().unwrap_or("Unknown".to_string());
        let user = get_current_username().unwrap();
        let username = user.to_string_lossy().to_string();

        Self {
            os,
            kernel_version,
            hostname,
            username,
        }
    }

    fn display(&self, color: &str, bold: bool) -> String {
        let reset = "\x1b[0m";
        let color_escape = match color {
            "black" => "\x1b[30m",
            "red" => "\x1b[31m",
            "green" => "\x1b[32m",
            "yellow" => "\x1b[33m",
            "blue" => "\x1b[34m",
            "magenta" => "\x1b[35m",
            "cyan" => "\x1b[36m",
            "light_black" => "\x1b[90m",
            "light_red" => "\x1b[91m",
            "light_green" => "\x1b[92m",
            "light_yellow" => "\x1b[93m",
            "light_blue" => "\x1b[94m",
            "light_magenta" => "\x1b[95m",
            "light_cyan" => "\x1b[96m",
            _ => "\x1b[32m",
        };

        let bold_escape = match bold {
            true => "\x1b[1m",
            false => "",
        };

        format!(
            "{bold_escape}{color_escape}{}{reset}{}{bold_escape}{color_escape}{}\nSystem: {reset}{}{bold_escape}{color_escape}\nKernel version: {reset}{}",
            self.username, "@", self.hostname, self.os, self.kernel_version
        )
    }
}

fn main() {
    let system_info = SystemInfo::new();
    println!("{}", system_info.display("green", true));
}
