use crate::essentials::process_grep;
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
            if process_grep(&comp) {
                return comp.to_string();
            }
        }

        "Unknown".to_string()
    } else if env::var("DISPLAY").is_ok() {
        let wm_output = String::from_utf8(Command::new("bash").arg("-c").arg(r#"id=$(xprop -root -notype _NET_SUPPORTING_WM_CHECK) && id=${id##* } && wm=$(xprop -id "$id" -notype -len 100 -f _NET_WM_NAME 8t) && wm=${wm/*WM_NAME = } && wm=${wm/\"} && wm=${wm/\"*} && printf $wm"#).output().unwrap().stdout)
            .expect("get_de_wm: Failed to convert stdout to string");
        wm_output
    } else {
        "Unknown".to_string()
    }
}
