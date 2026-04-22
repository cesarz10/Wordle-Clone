use colored::*;
use rand::seq::IteratorRandom;
use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader, Result};

// Using struct to propagate the instance of WordsDictionary instead of writing the filepath
pub struct WordsDictionary {
    pub dictionary_filepath: String,
}

// the dictionary struct can be instantiated in a function
impl WordsDictionary {
    // this way I can call WordsDictionary::new...
    pub fn new(file_path: String) -> Self {
        WordsDictionary {
            dictionary_filepath: file_path,
        }
    }
}

// creating another struct for a single dictionary entry
pub struct DictionaryEntry {
    word: String,
}

// Seems similar to the constructor of an object in python I guess
pub trait Dictionary {
    fn get_random_word(&self) -> Option<DictionaryEntry>;
    fn find_word(&self, text: &str) -> Option<DictionaryEntry>;
}

// Rudimentary implementation of the traits
impl Dictionary for WordsDictionary {
    // The first one is to get a random DictionaryEntry from a line of the Dictionary txt file
    fn get_random_word(&self) -> Option<DictionaryEntry> {
        let file_result: Result<File> = File::open(&self.dictionary_filepath);

        match file_result {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                let random_line = buf_reader.lines().choose(&mut rand::rng());

                match random_line {
                    Some(line) => Some(DictionaryEntry {
                        word: line.unwrap(),
                    }),
                    None => None,
                }
            }
            Err(error) => {
                println!("\nError reading from the dictionary: \n{}", error);

                None
            }
        }
    }

    // Search in the Dictionary for a specific entry - NO NECESSITY FOR THIS FUNCTION ->
    // WILL ONLY WORK TO LET THE USER KNOW THAT THE WORD DOES NOT EXIST IN THE DATABASE
    fn find_word(&self, text: &str) -> Option<DictionaryEntry> {
        let file_result: Result<File> = File::open(&self.dictionary_filepath);
        println!(
            "\nResult from reading the dictionary file:\n{:?}",
            file_result
        );

        match file_result {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                let mut word_option: Option<DictionaryEntry> = None;

                for line_result in buf_reader.lines() {
                    let line = line_result.unwrap();

                    if text.eq(line.trim()) {
                        // if the word in that line is the same as the one we are looking for (provided in parameters)
                        word_option = Some(DictionaryEntry {
                            word: String::from(line), // does this automatically trim the br line characters?
                        });

                        break;
                    }
                }

                word_option // if not found -> None
            }
            Err(error) => {
                println!(
                    "Error when looking for '{}' in the dictionary: \n{}",
                    text, error
                );

                None
            }
        }
    }
}

fn main() {
    // const GAME_WORD: &str = "JUANA";
    let dictionary = WordsDictionary::new(String::from("./words.txt"));
    let word_solution = dictionary.get_random_word().unwrap().word.to_uppercase();
    // println!("\nRANDOM WORD: {:?}\n", word_solution);

    let mut attempts: u32 = 0;
    let mut words = vec![];

    while attempts < 5 {
        attempts += 1;

        words.push(read_input(5));
        println!("\n\nAttempt #{}: {:?}", attempts, words.last().unwrap());

        // Looking at the user input and matching it with the GAME_WORD
        let feedback = match_words(words.last().unwrap(), &word_solution);

        if feedback {
            println!("\nYou got it!\nThe word of the day was {}\n", word_solution);
            return;
        } else {
            println!("\n{} attempts left!\n", 5-attempts);
            continue;
        }
    }

    println!("\nNo more attempts, good luck tomorrow!\n");
}

fn read_input(word_length: usize) -> String {
    let mut input: String = String::new();

    loop {
        println!("\nEnter a new word (or type 'exit()' to close the game):");

        input.clear(); // clean user input every time so the inputs don't stack and I just get the last string that was entered

        stdin().read_line(&mut input).unwrap();
        let clean = input.trim(); // getting rid of whitespaces

        // user can kill the program by typing "exit()" -> just easier for debugging
        if clean == "exit()" {
            println!("\nClosing the game, see you next time!\n");
            std::process::exit(0);
        }

        // for now it's a simple validating function checking on length. Will probably need to check
        // also that the word only has letters, no numbers or special characters
        if !validate_input(clean, word_length) {
            println!(
                "\nYour word must have 5 letters! The word you entered has {}. Try again.\n",
                clean.len()
            );
        } else {
            input = clean.to_uppercase();
            break;
        }
    }

    input // return the user input (uppercase)
}

fn validate_input(word: &str, expected_len: usize) -> bool {
    word.len() == expected_len
}

fn match_words(user_word: &str, solution: &String) -> bool {
    let user_chars: Vec<char> = user_word.chars().collect();
    let solution_chars: Vec<char> = solution.chars().collect();

    for i in 0..user_word.len() {
        let index: Option<usize> = solution.find(user_chars[i]);

        match index {
            Some(_index) => {
                if solution_chars[i].eq(&user_chars[i]) {
                    print!("{} ", user_chars[i].to_string().color("green"))
                } else {
                    print!("{} ", user_chars[i].to_string().color("yellow"))
                }
            }
            None => {
                print!("{} ", user_chars[i])
            }
        }
    }

    println!("");

    if String::from(user_word).eq(solution) {
        return true;
    }

    false
}
