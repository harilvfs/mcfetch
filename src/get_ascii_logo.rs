pub fn get_logo_by_system(system: &str) -> Vec<String> {
    let ascii_logo = match system {
        "Void" => vec![
            r#"    _______"#.to_string(),
            r#"    \_____ `-"#.to_string(),
            r#" /\   ___ `- \"#.to_string(),
            r#"| |  /   \  | |"#.to_string(),
            r#"| |  \___/  | |"#.to_string(),
            r#" \ `-_____  \/"#.to_string(),
            r#"  `-______\"#.to_string(),
        ],
        "Arch Linux" => vec![
            r#"      /\"#.to_string(),
            r#"     /  \"#.to_string(),
            r#"    /\   \"#.to_string(),
            r#"   /  __  \"#.to_string(),
            r#"  /  (  )  \"#.to_string(),
            r#" / __|  |__\\"#.to_string(),
            r#"/.`        `.\"#.to_string(),
        ],
        "Ubuntu" => vec![
            r#"         _"#.to_string(),
            r#"     ---(_)"#.to_string(),
            r#" _/  ---  \"#.to_string(),
            r#"(_) |   |"#.to_string(),
            r#"  \  --- _/"#.to_string(),
            r#"     ---(_)"#.to_string(),
        ],
        _ => vec![],
    };

    ascii_logo
}
