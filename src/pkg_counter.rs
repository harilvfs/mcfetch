use std::process::Command;

fn is_installed(command: &str) -> bool {
    let pkg = Command::new(command).output();

    match pkg {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub struct PackageManager {
    pub name: String,
    pub pkgs: u32,
}

impl PackageManager {
    pub fn get_pkg_manager() -> String {
        let pkg_managers = vec!["dnf", "pacman", "apt"];

        for m in pkg_managers {
            if is_installed(m) {
                return m.to_string();
            }
        }

        return "Unkonwn".to_string();
    }

    pub fn get_count() -> u32 {
        match PackageManager::get_pkg_manager().as_str() {
            "dnf" => {
                let output_raw = Command::new("dnf")
                    .arg("repoquery")
                    .arg("--userinstalled")
                    .arg("--qf")
                    .arg("%{name}")
                    .output()
                    .unwrap()
                    .stdout;
                let output = String::from_utf8(output_raw)
                    .expect("get_count: Failed to convert stdout to string");
                let mut count = 0;

                for _ in output.lines() {
                    count += 1;
                }
                count
            }
            "apt" => {
                let output_raw = Command::new("apt")
                    .arg("list")
                    .arg("--installed")
                    .output()
                    .unwrap()
                    .stdout;
                let output = String::from_utf8(output_raw)
                    .expect("get_count: Failed to convert stdout to string");
                let mut count = 0;

                for _ in output.lines() {
                    count += 1;
                }
                count
            }
            "pacman" => {
                let output_raw = Command::new("pacman").arg("-Q").output().unwrap().stdout;
                let output = String::from_utf8(output_raw)
                    .expect("get_count: Failed to convert stdout to string");
                let mut count = 0;

                for _ in output.lines() {
                    count += 1;
                }
                count
            }
            _ => 0,
        }
    }

    pub fn build() -> Self {
        Self {
            name: Self::get_pkg_manager(),
            pkgs: Self::get_count(),
        }
    }
}
