use sysinfo::System;
use users::get_current_username;
mod essentials;
mod pkg_counter;
use essentials::{get_formatting, get_kernel_info, get_shell, get_uptime};
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
}

impl SystemInfo {
    fn build() -> Self {
        let os = System::name().unwrap_or("Unknown".to_string());
        let kernel_version = get_kernel_info();
        let hostname = System::host_name().unwrap_or("Unknown".to_string());
        let user = get_current_username().unwrap();
        let username = user.to_string_lossy().to_string();
        let uptime = get_uptime();
        let PackageManager {
            name: pkg_manager_name,
            pkgs: pkg_count,
        } = PackageManager::build();
        let shell = get_shell();

        Self {
            os,
            kernel_version,
            hostname,
            username,
            uptime,
            pkg_count: pkg_count.to_string(),
            pkg_manager_name: pkg_manager_name.to_string(),
            shell,
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
{color_escape}SHELL: {reset}{}",
            self.username,
            "@",
            self.hostname,
            self.os,
            self.kernel_version,
            self.uptime,
            self.pkg_count,
            self.pkg_manager_name,
            self.shell
        )
    }
}

fn main() {
    let system_info = SystemInfo::build();
    println!("{}", system_info.display("green", true));
}
