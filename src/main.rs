use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

struct Main {
}


fn main(){
    let selected_word = selected_word();

    println!("The selected word was {}", selected_word)
}

fn selected_word() -> String {
    let mut file = File::open("words.txt").expect("Couldn't open the file");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Error occured while reading the file.");

    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    return String::from(available_words[random_index]);
 
}