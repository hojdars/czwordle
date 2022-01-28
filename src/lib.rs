use rand::Rng;

use std::fs;
use std::io;

extern crate colored;

use colored::*;

struct Guess {
    word: String,
    yellow_positions: Vec<u32>,
    green_positions: Vec<u32>,
}

struct State {
    tries: u32,
    word: String,
    guesses: Vec<Guess>,
}

struct Dictionary {
    wordlist: Vec<String>,
}

pub fn run(dictionary: &str) {
    let (mut state, dict) = setup(dictionary);

    guessing(&mut state, &dict);

    teardown(&state);
}

fn setup(path_to_dict: &str) -> (State, Dictionary) {
    println!("Guess a word!");

    let dictionary = load_dictionary(path_to_dict);

    let random_word = generate_random_word(&dictionary);

    (
        State {
            tries: 6,
            word: random_word.to_uppercase(),
            guesses: Vec::new(),
        },
        dictionary,
    )
}

fn guessing(state: &mut State, dictionary: &Dictionary) {
    for _ in 1..state.tries + 1 {
        let guess_word = input_word(dictionary);

        let guess = calculate_guess(&state.word, &guess_word);
        print_guess(&guess);

        state.guesses.push(guess);

        if state.word.eq(&guess_word) {
            break;
        }
    }
}

fn teardown(state: &State) {
    let is_won;
    match state.guesses.last() {
        None => panic!("Should not be tearing down with zero guesses, exit."),
        Some(guess) => is_won = guess.word.eq(&state.word),
    }

    if is_won {
        println!("Yay! {}", "You win!".green().bold());
    } else {
        println!(
            "You lose! :( The word was: {}",
            state.word.to_string().red().bold()
        );
    }
}

fn load_dictionary(path: &str) -> Dictionary {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
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

    Dictionary { wordlist: results }
}

fn generate_random_word(dict: &Dictionary) -> String {
    let num: usize = rand::thread_rng().gen_range(0..dict.wordlist.len().try_into().unwrap());
    dict.wordlist[num].clone()
}

fn input_word(dictionary: &Dictionary) -> String {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let word = input.trim();
                let letters = word.chars().count();
                if letters != 5 {
                    println!(
                        "Word needs to be 5 letters long! Your word was {} letters long!",
                        letters
                    );
                    input.clear();
                    continue;
                }
                if !dictionary.wordlist.contains(&word.to_string()) {
                    println!("{} is not in the dictionary!", word);
                    input.clear();
                    continue;
                }

                break;
            }
            Err(_) => panic!("Cannot read a line, exit."),
        }
    }
    input.trim().to_uppercase().to_owned()
}

fn calculate_guess(correct_word: &String, guess: &String) -> Guess {
    let mut result_guess = Guess {
        word: guess.clone(),
        yellow_positions: Vec::new(),
        green_positions: Vec::new(),
    };

    let correct_letters: Vec<char> = correct_word.chars().collect();
    let guess_letters: Vec<char> = guess.chars().collect();

    assert!(correct_letters.len() == guess_letters.len());

    let mut correct_it = correct_letters.iter();
    let mut guess_it = guess_letters.iter();

    for i in 0..correct_letters.len() {
        let correct_char = correct_it.next().unwrap();
        let guess_char = guess_it.next().unwrap();

        if correct_char == guess_char {
            result_guess.green_positions.push(i.try_into().unwrap());
            continue;
        }

        if correct_letters.contains(&guess_char) {
            result_guess.yellow_positions.push(i.try_into().unwrap());
            continue;
        }
    }

    result_guess
}

fn print_guess(guess: &Guess) {
    let mut i: u32 = 0;
    for char in guess.word.chars() {
        if guess.green_positions.contains(&i) {
            print!("{} ", char.to_string().to_uppercase().green());
        } else if guess.yellow_positions.contains(&i) {
            print!("{} ", char.to_string().to_uppercase().yellow());
        } else {
            print!("{} ", char.to_string().to_uppercase());
        }
        i = i + 1;
    }
    print!("\n");
}
