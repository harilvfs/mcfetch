use crate::essentials::process_grep;
use regex::Regex;
use std::env;
use std::process::Command;

pub fn main() -> String {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        let wl_comps = vec![
            "arcan",
            "asc",
            "clayland",
            "dwc",
            "fireplace",
            "gnome-shell",
            "greenfield",
            "grefsen",
            "hyprland",
            "kwin",
            "lipstick",
            "maynard",
            "mazecompositor",
            "niri",
            "motorcar",
            "orbital",
            "orbment",
            "perceptia",
            "rustland",
            "sway",
            "ulubis",
            "velox",
            "wavy",
            "way-cooler",
            "wayfire",
            "wayhouse",
            "westeros",
            "westford",
            "weston",
        ];

        for comp in wl_comps {
            if process_grep(comp) {
                return comp.to_string();
            }
        }

        "Unknown".to_string()
    } else if env::var("DISPLAY").is_ok() {
        let wm_id_re = Regex::new(r"0x[0-9a-fA-F]+").unwrap();
        let wm_id_command = String::from_utf8(
            Command::new("xprop")
                .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
                .output()
                .unwrap()
                .stdout,
        )
        .expect("wm_id_command: Cannot convert bytes to UTF8");
        let wm_id = wm_id_re.captures(wm_id_command.as_str()).unwrap();

        let wm_name_re = Regex::new(r#"_NET_WM_NAME\s*=\s*"([^"]+)""#).unwrap();
        let wm_name_command = String::from_utf8(
            Command::new("xprop")
                .args([
                    "-id",
                    &wm_id[0],
                    "-notype",
                    "-len",
                    "100",
                    "-f",
                    "_NET_WM_NAME",
                    "8t",
                ])
                .output()
                .unwrap()
                .stdout,
        )
        .expect("wm_name_command: Cannot convert bytes to UTF8");
        let wm_name = wm_name_re.captures(wm_name_command.as_str()).unwrap();
        wm_name[1].to_string()
    } else {
        "Unknown".to_string()
    }
}
