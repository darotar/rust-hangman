use crate::letter::Letter;
use crate::game_status::GameStatus;

pub struct Progress {}

impl Progress {
  pub fn display(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }
    }

    println!("{}", display_string);
  }

  pub fn check(turns_left: u8, letters: &Vec<Letter>) -> GameStatus {
    let mut all_revealed = true;

    for letter in letters {
      if !letter.revealed {
        all_revealed = false;
      }
    }

    if all_revealed {
      return GameStatus::Won;
    }

    if turns_left > 0 {
      return GameStatus::InProgress;
    }

    GameStatus::Lost
  }
}