use std::collections::HashSet;

pub struct Letters {
    used: HashSet<char>,
    green: HashSet<char>,
    yellow: HashSet<char>,
}

impl Letters {
    pub fn new() -> Letters {
        Letters {
            used: HashSet::new(),
            green: HashSet::new(),
            yellow: HashSet::new(),
        }
    }

    pub fn add_used_letter(&mut self, letter: char) {
        self.used.insert(letter);
    }

    pub fn add_green_letter(&mut self, letter: char) {
        self.green.insert(letter);
    }

    pub fn add_yellow_letter(&mut self, letter: char) {
        self.yellow.insert(letter);
    }

    pub fn get_used_letters(&self) -> &HashSet<char> {
        &self.used
    }

    pub fn get_green_letters(&self) -> &HashSet<char> {
        &self.green
    }

    pub fn get_yellow_letters(&self) -> &HashSet<char> {
        &self.yellow
    }
}
