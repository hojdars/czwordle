use rand::Rng;
use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub struct Dictionary {
    wordlist: Vec<String>,
    wordset: HashSet<String>,
}

impl Dictionary {
    pub fn new(text_file: &str, word_length: u32) -> Dictionary {
        let mut result = Dictionary {
            wordlist: Vec::new(),
            wordset: HashSet::new(),
        };

        for line in text_file.lines() {
            let word_it = line.split('/').next();
            if let Some(string) = word_it {
                let string_no_whitespace = string.trim();
                if string_no_whitespace.chars().count() != word_length.try_into().unwrap() {
                    continue;
                }

                if string_no_whitespace.chars().next().unwrap().is_uppercase() {
                    continue;
                }

                result.wordlist.push(string_no_whitespace.to_uppercase());
                result.wordset.insert(string_no_whitespace.to_uppercase());
            }
        }

        result
    }

    pub fn get_random_word(&self) -> String {
        let num: usize = rand::thread_rng().gen_range(0..self.wordlist.len().try_into().unwrap());
        self.wordlist[num].clone()
    }

    pub fn contains(&self, word: &str) -> bool {
        self.wordset.contains(&word.to_uppercase())
    }
}
