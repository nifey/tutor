use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;

pub fn get_section(f: &File, section: &str) -> Result<String, String> {
    let mut reader = BufReader::new(f);
    let mut text = String::new();
    let mut line = String::new();
    let mut text_reached = false;
    let pattern: String = "===".to_string() + section;
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if text_reached && line.starts_with("===") {
            break;
        } else if text_reached {
            text += &line;
        } else if line.starts_with(pattern.as_str()) {
            text_reached = true;
        }
        line.clear();
        if bytes_read == 0 {
            break;
        }
    }
    if text.len() == 0 {
        Err("Cannot find section in file".to_string())
    } else {
        Ok(text)
    }
}

pub fn check(shell_script: String, file: &str) -> bool {
    let shell_script = shell_script.replace("<<file>>", file);
    let commands: Vec<&str> = shell_script.trim().split(" ").collect();
    Command::new("sh")
        .args(commands)
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}
