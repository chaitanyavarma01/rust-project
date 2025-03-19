use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character:char,
    revealed:bool,
}

enum GameProgress{
    InProgress,
    Won,
    Lost
}


fn main(){
    let selected_word = selected_word();
    let mut turns_left = ALLOWED_ATTEMPTS;
    let mut letters = create_letter(&selected_word);

    println!("Welcome to Hangman!");
    loop {
        println!("You have {} turns left", turns_left);
        display_function(&letters);

        println!("Please enter a letter to guess:");
        let user_char = read_user_input();

        if user_char == '*'{
            break;
        }

        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut(){
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }

        }

        if !at_least_one_revealed{
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats! You won! ☺");
                break;
            }
            GameProgress::Lost => {
                println!("\nYou lost! ☹");
                break;
            }
        }
    }
    println!("\nGoodbye!");
}

fn selected_word() -> String {
    let mut file = File::open("words.txt").expect("Couldn't open the file");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Error occured while reading the file.");

    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    return String::from(available_words[random_index]);
 
}

fn create_letter(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars(){
        letters.push(Letter{
            character : c,
            revealed : false,
        })
    }
    return letters;

}

fn display_function(letters:&Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }
        display_string.push(' ');
    }
    println!("{}", display_string);
}

fn read_user_input() -> char {
    let mut user_input =  String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    /* Determine if all letters have been revealed */
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}