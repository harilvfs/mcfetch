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

    fn display(&self) -> String {
        format!(
            "
        {}{}{}
        System: {}
        Kernel version: {}
        ",
            self.username, "@", self.hostname, self.os, self.kernel_version
        )
    }
}

fn main() {
    let system_info = SystemInfo::new();
    println!("{}", system_info.display());
}
