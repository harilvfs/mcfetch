use sysinfo::System;
mod essentials;
mod get_de_wm;
mod pkg_counter;
use essentials::*;
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

        format!(
            "{color_escape}{}{reset}{}{color_escape}{}
OS: {reset}{}
{color_escape}KERNEL: {reset}{}
{color_escape}UPTIME: {reset}{}
{color_escape}PACKAGES: {reset}{} ({})
{color_escape}SHELL: {reset}{}
{color_escape}UI: {reset}{}",
            self.username,
            "@",
            self.hostname,
            self.os,
            self.kernel_version,
            self.uptime,
            self.pkg_count,
            self.pkg_manager_name,
            self.shell,
            self.ui
        )
    }
}

fn main() {
    let system_info = SystemInfo::build();
    println!("{}", system_info.display("green", true));
}
