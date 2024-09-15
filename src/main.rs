use sysinfo::System;
use users::get_current_username;

struct SystemInfo {
    os: String,
    kernel_version: String,
    hostname: String,
    username: String,
}

fn get_formatting(format: &str) -> String {
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

impl SystemInfo {
    fn build() -> Self {
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
        let reset = get_formatting("reset");
        let color_escape = match (bold, color) {
            (false, c) => get_formatting(c),
            (true, c) => format!("{}{}", get_formatting("bold"), get_formatting(c)),
        };

        format!(
            "{color_escape}{}{reset}{}{color_escape}{}\nSystem: {reset}{}{color_escape}\nKernel version: {reset}{}",
            self.username, "@", self.hostname, self.os, self.kernel_version
        )
    }
}

fn main() {
    let system_info = SystemInfo::build();
    println!("{}", system_info.display("green", true));
}
