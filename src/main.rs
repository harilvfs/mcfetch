use sysinfo::System;
mod essentials;
mod get_ascii_logo;
mod get_de_wm;
mod memory;
mod pkg_counter;
mod terminal;
use clap::Parser;
use essentials::*;
use get_ascii_logo::*;
use memory::get_memory_info;
use pkg_counter::PackageManager;
use std::cmp::Ordering;
use terminal::get_terminal;

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
    terminal: String,
    memory_used: String,
    memory_total: String,
    os_age: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Color of the output (red, yellow, blue etc.)
    #[arg(short, long, default_value_t = String::from("green"))]
    color: String,

    /// Use normal formatting instead of bold
    #[arg(short, long, default_value_t = false)]
    normal: bool,
}

impl SystemInfo {
    fn build() -> Self {
        let PackageManager {
            name: pkg_manager_name,
            pkgs: pkg_count,
        } = PackageManager::build();

        let memory_info = get_memory_info();

        Self {
            os: System::name().unwrap_or("Unknown".to_string()),
            kernel_version: get_kernel_info(),
            hostname: System::host_name().unwrap_or("Unknown".to_string()),
            username: get_username(),
            pkg_count: pkg_count.to_string(),
            pkg_manager_name: pkg_manager_name.to_string(),
            shell: get_shell(),
            terminal: get_terminal(),
            ui: get_de_wm::main(),
            memory_used: memory_info.used,
            memory_total: memory_info.total,
            os_age: get_os_age(),
            uptime: get_uptime(),
        }
    }

    fn display(&self, color: String, bold: bool) -> String {
        let reset = get_formatting("reset");
        let color_escape = match (bold, color) {
            (false, c) => get_formatting(&c),
            (true, c) => format!("{}{}", get_formatting("bold"), get_formatting(&c)),
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
            format!("{color_escape}TERMINAL: {reset}{}", self.terminal),
            format!("{color_escape}UI: {reset}{}", self.ui),
            format!(
                "{color_escape}MEMORY: {reset}{} / {}",
                self.memory_used, self.memory_total
            ),
            format!("{color_escape}OS AGE: {reset}{}", self.os_age),
            format!("{color_escape}UPTIME: {reset}{}", self.uptime),
        ];

        let mut logo = get_logo_by_system(&self.os);

        match info.len().cmp(&logo.len()) {
            Ordering::Greater => {
                for _ in 0..(info.len() - logo.len()) {
                    logo.push("".to_string());
                }
            }
            Ordering::Less => {
                for _ in 0..(logo.len() - info.len()) {
                    info.push("".to_string());
                }
            }
            Ordering::Equal => {}
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
    let args = Args::parse();
    let system_info = SystemInfo::build();
    print!("{}", system_info.display(args.color, !args.normal));
}
