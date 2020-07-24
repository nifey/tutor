use super::index::{new_from_string, Index};
use serde::Deserialize;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Lesson {
    title: String,
    file: String,
}

impl Lesson {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }
}

#[derive(Deserialize, Debug)]
struct Section {
    title: String,
    lessons: Vec<Lesson>,
}

impl Section {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_lesson_title(&self, lesson_index: u32) -> Result<String, String> {
        if self.lessons.len() as u32 <= lesson_index {
            Ok(self.lessons[lesson_index as usize - 1].get_title())
        } else {
            Err("Lesson does not exist".to_string())
        }
    }

    pub fn get_lesson_file(&self, lesson_index: u32) -> Result<String, String> {
        if lesson_index <= self.lessons.len() as u32 {
            Ok(self.lessons[lesson_index as usize - 1].get_file())
        } else {
            Err("Lesson does not exist".to_string())
        }
    }

    pub fn lesson_exists(&self, lesson_index: u32) -> bool {
        if lesson_index <= self.lessons.len() as u32 {
            true
        } else {
            false
        }
    }

    pub fn get_num_lessons(&self) -> u32 {
        self.lessons.len().try_into().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Info {
    title: String,
    version: String,
    instructions: String,
    sections: Vec<Section>,
}

impl Info {
    pub fn get_instructions(&self) -> String {
        self.instructions.clone()
    }

    pub fn get_lesson_file(&self, lesson: String) -> Result<String, String> {
        let lesson_index: Index = new_from_string(lesson)?;
        let section = lesson_index.get_section();
        if section as usize <= self.sections.len() {
            self.sections[section as usize - 1].get_lesson_file(lesson_index.get_lesson())
        } else {
            Err("Section Not found".to_string())
        }
    }

    pub fn lesson_exists(&self, lesson: String) -> bool {
        if let Ok(lesson_index) = new_from_string(lesson) {
            let section = lesson_index.get_section();
            if section as usize <= self.sections.len() {
                self.sections[section as usize - 1].lesson_exists(lesson_index.get_lesson())
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_total_lessons(&self) -> u32 {
        self.sections
            .iter()
            .map(|x| x.get_num_lessons())
            .fold(0, |a, b| a + b)
    }

    pub fn get_tutorial_version(&self) -> String {
        self.version.clone()
    }

    pub fn get_next_lesson(&self, index: Index) -> Option<String> {
        let current_section = index.get_section();
        let current_lesson = index.get_lesson();
        let next_lesson_in_section: String =
            current_section.to_string() + "." + &(current_lesson + 1).to_string();
        let next_section: String =
            (current_section + 1).to_string() + "." + &current_lesson.to_string();
        if self.lesson_exists(next_lesson_in_section.clone()) {
            Some(next_lesson_in_section)
        } else if self.lesson_exists(next_section.clone()) {
            Some(next_section)
        } else {
            None
        }
    }
}

pub fn read_tutorinfo() -> Info {
    let mut info_file = File::open("tutorinfo.toml")
        .expect("Run tutor from a tutorial directory containing tutorinfo.toml file");
    let mut contents = String::new();
    info_file
        .read_to_string(&mut contents)
        .expect("Error reading tutorinfo.toml file");
    toml::from_str::<Info>(contents.as_str()).expect("Error parsing tutorinfo.toml file")
}
