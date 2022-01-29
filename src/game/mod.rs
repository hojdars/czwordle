use super::dictionary;
use super::letters::Letters;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Guess {
    pub is_correct: bool,
    pub word: String,
    pub yellow_positions: Vec<u32>,
    pub green_positions: Vec<u32>,
}

#[derive(Debug, PartialEq)]
pub enum GuessError {
    NotInDictionary,
    WrongLength(u32),
}

#[derive(PartialEq)]
pub enum GameState {
    Win(u32),
    Lose,
    Ongoing(u32),
}

pub struct Game<'dict> {
    state: State,

    dictionary: &'dict dictionary::Dictionary,
}

struct State {
    maximum_tries: u32,
    word_to_guess: String,
    guesses: Vec<Guess>,
    letters: Letters,
}

impl<'d> Game<'d> {
    pub fn new(maximum_tries: u32, dictionary: &'d dictionary::Dictionary) -> Game<'d> {
        Game {
            state: State {
                maximum_tries,
                word_to_guess: dictionary.get_random_word(),
                guesses: Vec::new(),
                letters: Letters::new(),
            },
            dictionary,
        }
    }

    pub fn get_game_state(&self) -> GameState {
        let guess_count: u32 = self.state.guesses.len().try_into().unwrap();
        let mut was_last_guess_winning = false;

        if guess_count > 0 {
            was_last_guess_winning = self.state.guesses.last().unwrap().is_correct;
        }

        if was_last_guess_winning {
            return GameState::Win(guess_count);
        }

        let are_guesses_depleted = guess_count == self.state.maximum_tries.try_into().unwrap();

        if are_guesses_depleted {
            return GameState::Lose;
        } else {
            return GameState::Ongoing(guess_count);
        }
    }

    pub fn get_correct_word(&self) -> String {
        // TODO: only return correct word when game is finished
        self.state.word_to_guess.clone()
    }

    pub fn submit_guess(&mut self, guessed_word: &str) -> Result<Guess, GuessError> {
        if guessed_word.chars().count() != self.state.word_to_guess.chars().count() {
            return Err(GuessError::WrongLength(
                guessed_word.chars().count().try_into().unwrap(),
            ));
        }

        if !self.dictionary.contains(guessed_word) {
            return Err(GuessError::NotInDictionary);
        }

        let guess = self.calculate_guess(guessed_word);
        self.state.guesses.push(guess.clone());
        Ok(guess.clone())
    }

    pub fn get_letters(&self) -> &Letters {
        &self.state.letters
    }

    pub fn get_guesses(&self) -> &Vec<Guess> {
        &self.state.guesses
    }

    fn calculate_guess(&mut self, guessed_word: &str) -> Guess {
        let mut result_guess = Guess {
            is_correct: false,
            word: guessed_word.to_uppercase(),
            yellow_positions: Vec::new(),
            green_positions: Vec::new(),
        };

        if result_guess.word == self.state.word_to_guess {
            result_guess.is_correct = true;
        }

        let correct_letters: Vec<char> = self.state.word_to_guess.chars().collect();
        let guess_letters: Vec<char> = result_guess.word.chars().collect();

        assert!(correct_letters.len() == guess_letters.len());

        let mut correct_it = correct_letters.iter();
        let mut guess_it = guess_letters.iter();

        for i in 0..correct_letters.len() {
            let correct_char = correct_it.next().unwrap();
            let guess_char = guess_it.next().unwrap();

            self.state.letters.add_used_letter(*guess_char);

            if correct_char == guess_char {
                result_guess.green_positions.push(i.try_into().unwrap());
                self.state.letters.add_green_letter(*guess_char);
                continue;
            }

            if correct_letters.contains(&guess_char) {
                result_guess.yellow_positions.push(i.try_into().unwrap());
                self.state.letters.add_yellow_letter(*guess_char);
                continue;
            }
        }

        result_guess
    }
}
