use sysinfo::System;
mod essentials;
mod get_ascii_logo;
mod get_de_wm;
mod pkg_counter;
use essentials::*;
use get_ascii_logo::*;
use pkg_counter::PackageManager;

struct SystemInfo {
    os: String,
    kernel_version: String,
    hostname: String,
    username: String,
    uptime: String,
    pkg_count: String,
    pkg_manager_name: String,
    shell: String,
    ui: String,
}

impl SystemInfo {
    fn build() -> Self {
        let PackageManager {
            name: pkg_manager_name,
            pkgs: pkg_count,
        } = PackageManager::build();

        Self {
            os: System::name().unwrap_or("Unknown".to_string()),
            kernel_version: get_kernel_info(),
            hostname: System::host_name().unwrap_or("Unknown".to_string()),
            username: get_username(),
            uptime: get_uptime(),
            pkg_count: pkg_count.to_string(),
            pkg_manager_name: pkg_manager_name.to_string(),
            shell: get_shell(),
            ui: get_de_wm::main(),
        }
    }

    fn display(&self, color: &str, bold: bool) -> String {
        let reset = get_formatting("reset");
        let color_escape = match (bold, color) {
            (false, c) => get_formatting(c),
            (true, c) => format!("{}{}", get_formatting("bold"), get_formatting(c)),
        };
        let mut result = String::new();

        let mut info = vec![
            format!(
                "{color_escape}{}{reset}@{color_escape}{}",
                self.username, self.hostname
            ),
            format!("{color_escape}OS: {reset}{}", self.os),
            format!("{color_escape}KERNEL: {reset}{}", self.kernel_version),
            format!("{color_escape}UPTIME: {reset}{}", self.uptime),
            format!(
                "{color_escape}PACKAGES: {reset}{} ({})",
                self.pkg_count, self.pkg_manager_name
            ),
            format!("{color_escape}SHELL: {reset}{}", self.shell),
            format!("{color_escape}UI: {reset}{}", self.ui),
        ];

        let mut logo = get_logo_by_system(&self.os);

        if info.len() > logo.len() {
            for _ in 0..(info.len() - logo.len()) {
                logo.push("".to_string());
            }
        } else if logo.len() > info.len() {
            for _ in 0..(logo.len() - info.len()) {
                info.push("".to_string());
            }
        }

        let logo_max_line_length = logo.iter().map(|s| s.len()).max().unwrap_or(0);

        for it in logo.into_iter().zip(info) {
            let (logo_line, info_line) = it;
            let logo_line_length = logo_line.chars().count();
            let line = format!(
                "{color_escape}{}{reset}{: >length$}{}\n",
                logo_line,
                "",
                info_line,
                length = logo_max_line_length + 3 - logo_line_length
            );
            result.push_str(&line);
        }

        result
    }
}

fn main() {
    let system_info = SystemInfo::build();
    print!("{}", system_info.display("green", true));
}
