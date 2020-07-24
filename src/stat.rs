use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    tutorial_version: String,
    current_lesson: String,
    finished_lessons: Vec<String>,
    total_lessons: u32,
    hints_used: HashMap<String, u32>,
}

impl Stat {
    pub fn get_tutorial_version(&self) -> String {
        self.tutorial_version.clone()
    }

    pub fn get_current_lesson(&self) -> String {
        self.current_lesson.clone()
    }

    pub fn get_hints_used(&self, lesson: String) -> Result<u32, String> {
        //TODO check if the lesson is a valid string and return error if invalid
        if self.hints_used.contains_key(&lesson) {
            Ok(*self.hints_used.get(&lesson).unwrap())
        } else {
            Ok(0)
        }
    }

    pub fn set_current_lesson(&mut self, lesson: String) {
        self.current_lesson = lesson;
    }
}

pub fn new(tutorial_version: String, current_lesson: String, total_lessons: u32) -> Stat {
    Stat {
        tutorial_version,
        current_lesson,
        finished_lessons: Vec::<String>::new(),
        total_lessons,
        hints_used: HashMap::<String, u32>::new(),
    }
}

pub fn read_tutorstat() -> Option<Stat> {
    if let Ok(mut stat_file) = File::open("tutorstat.toml") {
        let mut contents = String::new();
        stat_file
            .read_to_string(&mut contents)
            .expect("Error reading tutorstat.toml file");
        Some(toml::from_str::<Stat>(contents.as_str()).expect("Error parsing tutorstat.toml file"))
    } else {
        println!("tutorstat.toml file not found");
        None
    }
}

pub fn write_tutorstat(stat: Stat) {
    let mut stat_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("tutorstat.toml")
        .expect("Cannot open tutorstat.toml for writing");
    stat_file.write_all(toml::to_string_pretty(&stat).unwrap().as_bytes());
}
