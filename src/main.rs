use czwordle::run;

use std::fs;

fn main() {
    // run();

    let contents = fs::read_to_string("jmena.txt").expect("Something went wrong reading the file");

    let mut results: Vec<String> = Vec::new();

    for line in contents.lines() {
        let word_it = line.split('/').next();
        if let Some(string) = word_it {
            let string_no_whitespace = string.trim();
            if string_no_whitespace.chars().count() != 5 {
                continue;
            }

            if string_no_whitespace.chars().next().unwrap().is_uppercase() {
                continue;
            }

            results.push(string_no_whitespace.to_string());
        }
    }

    println!("{:?}", results);
    println!("{}", results.len());
}
