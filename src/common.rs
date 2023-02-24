use std::{env, fs::read_to_string};

use anyhow::Result;

pub fn read_lines(day: &str) -> Result<Vec<String>> {
    // for `instruments.app`
    // let file_name = env::current_exe()?
    //     .parent() // debug / release
    //     .unwrap()
    //     .parent() // target
    //     .unwrap()
    //     .join(format!("../input/d{}.txt", day))
    //     .canonicalize()?;
    let file_name = format!("input/d{}.txt", day);
    let content = read_to_string(file_name)?;
    let mut lines: Vec<String> = content.split("\n").map(|s| s.to_owned()).collect();
    if lines.last() == Some(&"".to_string()) {
        lines.pop();
    }
    Ok(lines)
}
