use std::fs::File;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use std::io;

mod letter;
mod progress;
mod game_status;
mod consts;

use letter::Letter;
use progress::Progress;
use game_status::GameStatus;
use consts::ALLOWED_ATTEMPTS;

fn main() {
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    let mut turns_left = ALLOWED_ATTEMPTS;

    loop {
        println!("You have {} turns left", turns_left);
        Progress::display(&letters);

        println!("Please enter a letter to guess");
        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed = false;

        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match Progress::check(turns_left, &letters) {
            GameStatus::Won => {
                println!("Congrats, you won, your word was {}", selected_word);
                break;
            },
            GameStatus::Lost => {
                println!("Sorry, you failed");
                break;
            },
            GameStatus::InProgress => continue,
        }
    }
}

fn select_word() -> String {
    let mut file = File::open("words.txt")
        .expect("Could not open file!");
    let mut file_content = String::new();

    file
        .read_to_string(&mut file_content)
        .expect("An error has occured while reading the file");

    let available_words: Vec<&str> = file_content
        .trim()
        .split(", ")
        .collect();

    let random_index = thread_rng().gen_range(0..available_words.len());

    String::from(available_words[random_index])
}

fn create_letters(word: &String) -> Vec<Letter> {
    word
        .chars()
        .map(|c| Letter {
            character: c,
            revealed: false,
        })
        .collect()
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => c,
                None => '*'
            }
        },
        Err(_) => '*'
    }
}