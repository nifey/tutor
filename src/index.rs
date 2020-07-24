pub struct Index {
    index: String,
}

impl Index {
    pub fn get_string(&self) -> String {
        self.index.clone()
    }

    pub fn get_section(&self) -> u32 {
        self.index
            .split(".")
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    pub fn get_lesson(&self) -> u32 {
        self.index
            .split(".")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }
}

pub fn new(section: u32, lesson: u32) -> Index {
    Index {
        index: section.to_string() + "." + &lesson.to_string(),
    }
}

pub fn new_from_string(string: String) -> Result<Index, String> {
    let parts: Vec<&str> = string.split(".").collect();
    if parts.len() == 2
        && parts
            .iter()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .is_ok()
    {
        Ok(Index { index: string })
    } else {
        Err("Invalid Index".to_string())
    }
}
