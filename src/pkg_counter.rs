use std::collections::HashMap;
use std::iter::FromIterator;
use std::process::Command;

fn is_installed(command: &str) -> bool {
    let (program, args) = command.split_once(char::is_whitespace).unwrap();
    let args: Vec<&str> = args.split(char::is_whitespace).collect();
    let pkg = Command::new(program).args(&args).output();

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
        let pkg_managers: HashMap<&str, &str> = HashMap::from_iter([
            ("pacman", "pacman -V"),
            ("apk", "apk -V"),
            ("xbps", "xbps-query -V"),
            ("dnf", "dnf --veresion"),
            ("apt", "apt -v"),
        ]);

        for (name, try_command) in &pkg_managers {
            if is_installed(try_command) {
                return name.to_string();
            }
        }

        return "Unkonwn".to_string();
    }

    pub fn get_count() -> u32 {
        match PackageManager::get_pkg_manager().as_str() {
            "xbps" => {
                let output_raw = Command::new("xbps-query")
                    .arg("-l")
                    .output()
                    .unwrap()
                    .stdout;
                let output = String::from_utf8(output_raw)
                    .expect("get_count: Failed to convert stdout to string");
                let mut count = 0;

                for line in output.lines() {
                    if line.starts_with("ii") {
                        count += 1;
                    }
                }
                count
            }
            "apk" => {
                let output_raw = Command::new("apk")
                    .arg("list")
                    .arg("-I")
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
            "pacman" => {
                let output_raw = Command::new("pacman").arg("-Qq").output().unwrap().stdout;
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
