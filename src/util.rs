use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;

pub fn get_section(filename: &str, section: &str) -> Result<String, String> {
    if let Ok(lesson_file) = File::open(filename) {
        let mut reader = BufReader::new(lesson_file);
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
    } else {
        Err("Cannot open file".to_string())
    }
}

pub fn get_n_hints(hints: String, n: u32) -> Result<(String, u32), String> {
    let mut text = String::new();
    let mut num_hints = 0;
    for line in hints.lines() {
        if num_hints > n {
            break;
        } else if line.starts_with("==hint") {
            num_hints += 1;
        } else {
            text += &line;
            text += "\n";
        }
    }
    if text.len() == 0 {
        Err("Cannot find section in file".to_string())
    } else {
        Ok((text, num_hints))
    }
}

pub fn check(shell_script: String, file: &str) -> bool {
    let shell_script = shell_script.replace("<<file>>", file);
    if let Some((interpreter, arguments)) = shell_script
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .split_first()
    {
        Command::new(interpreter)
            .args(arguments)
            .spawn()
            .and_then(|mut child| child.wait())
            .map(|status| status.success())
            .unwrap_or(false)
    } else {
        false
    }
}
