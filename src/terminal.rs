use std::env;
use std::process::Command;

pub fn get_terminal() -> String {
    if env::var("TMUX").is_ok() {
        if let Ok(output) = Command::new("tmux").arg("-V").output() {
            if let Ok(version) = String::from_utf8(output.stdout) {
                return version.trim().to_string();
            }
        }
        return "tmux (unknown version)".to_string();
    }

    if let Ok(term) = env::var("TERM") {
        if let Some(stripped) = term.strip_prefix("xterm-") {
            return stripped.to_string();
        }
        return term;
    }

    "Unknown".to_string()
}
