use std::collections::HashSet;

extern crate colored;
use colored::*;

use super::game::Guess;
use super::letters::Letters;

pub fn print_win(tries: u32) {
    println!(
        "Yay! {}",
        format!("You win in {} tries!", tries)
            .to_string()
            .green()
            .bold()
    );
}

pub fn print_lose(correct_word: &str) {
    println!(
        "You lose! :( The word was: {}",
        correct_word.to_string().red().bold()
    );
}

pub fn print_guesses(guesses: &Vec<Guess>) {
    for g in guesses {
        print_guess(g);
    }
}

pub fn print_letters(letters: &Letters) {
    print_alphabet(
        letters.get_used_letters(),
        letters.get_green_letters(),
        letters.get_yellow_letters(),
    )
}

fn print_guess(guess: &Guess) {
    let mut i: u32 = 0;
    for char in guess.word.chars() {
        if guess.green_positions.contains(&i) {
            print!("{} ", char.to_string().to_uppercase().green().bold());
        } else if guess.yellow_positions.contains(&i) {
            print!("{} ", char.to_string().to_uppercase().yellow().bold());
        } else {
            print!("{} ", char.to_string().to_uppercase());
        }
        i = i + 1;
    }
    print!("\n");
}

fn print_letter(
    letter: char,
    used_letters: &HashSet<char>,
    green_letters: &HashSet<char>,
    yellow_letters: &HashSet<char>,
) {
    let letter: char = letter.to_uppercase().next().unwrap();
    if green_letters.contains(&letter) {
        print!("{}", letter.to_string().green().bold());
    } else if yellow_letters.contains(&letter) {
        print!("{}", letter.to_string().yellow().bold());
    } else if used_letters.contains(&letter) {
        print!("{}", letter.to_string().hidden());
    } else {
        print!("{}", letter.to_string());
    }
}

fn print_all_row(
    used_letters: &HashSet<char>,
    green_letters: &HashSet<char>,
    yellow_letters: &HashSet<char>,
) {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for p in alphabet.chars() {
        print_letter(p, used_letters, green_letters, yellow_letters);
        print!(" ");
    }
}

fn print_custom_row(
    row: &[(char, i32)],
    used_letters: &HashSet<char>,
    green_letters: &HashSet<char>,
    yellow_letters: &HashSet<char>,
) {
    let mut last_post: i32 = 0;
    for (p, pos) in row {
        let pocet_mezer = (pos - last_post) * 2 - 1;
        for _ in 0..pocet_mezer {
            print!(" ");
        }
        print_letter(*p, used_letters, green_letters, yellow_letters);
        last_post = *pos;
    }
}

fn print_alphabet(
    used_letters: &HashSet<char>,
    green_letters: &HashSet<char>,
    yellow_letters: &HashSet<char>,
) {
    let hacky_carky = [
        ('á', 0),
        ('č', 2),
        ('ď', 3),
        ('ě', 4),
        ('í', 8),
        ('ň', 13),
        ('ó', 14),
        ('ř', 17),
        ('š', 18),
        ('ť', 19),
        ('ú', 20),
        ('ý', 24),
        ('ž', 25),
    ];
    let hacky_carky_dve = [('é', 4), ('ů', 20)];

    print_all_row(used_letters, green_letters, yellow_letters);
    print!("\n");

    print_custom_row(&hacky_carky, used_letters, green_letters, yellow_letters);
    print!("\n");

    print!(" ");
    print_custom_row(
        &hacky_carky_dve,
        used_letters,
        green_letters,
        yellow_letters,
    );
    print!("\n");
}
