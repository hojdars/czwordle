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

fn input_word() -> String {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let letters = input.trim().chars().count();
                if letters == 5 {
                    break;
                } else {
                    println!(
                        "Word needs to be 5 letters long! Your word was {} letters long!",
                        letters
                    );
                    input.clear();
                }
            }
            Err(_) => panic!("Cannot read a line, exit."),
        }
    }
    input.trim().to_uppercase().to_owned()
}

pub fn run() {
    let mut state = setup();

    guessing(&mut state);

    teardown(&state);
}

fn setup() -> State {
    println!("Guess a word!");

    State {
        tries: 6,
        word: String::from("tajne").to_uppercase(),
        guesses: Vec::new(),
    }
}

fn guessing(state: &mut State) {
    for _ in 1..state.tries + 1 {
        let guess_word = input_word();

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
        println!("Gratz! {}", "You win!".green().bold());
    } else {
        println!(
            "You lose! :( The word was: {}",
            state.word.to_string().red().bold()
        );
    }
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

    let mut correct_it = correct_word.chars();
    let mut guess_it = guess.chars();

    for i in 0..correct_word.len() {
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
