//use std::fs;
// use std::fs::File;
use colored::{ColoredString, Colorize};
use std::io;
use std::io::prelude::*;

// use std::error::Error;
use rand::Rng;

fn check_answer(mut grid: Vec<String>) -> Result<Vec<(char, i32)>, std::io::Error> {
    // pop the answer and entry words off th Vec.
    let entry = grid.pop().unwrap();
    let answer: String = grid.pop().unwrap();
    //let answer: String = grid[i].unwrap();

    // define a Vec with a (char, i32) that will hold the return
    let mut inchars: Vec<(char, i32)> = Vec::new();

    if answer.eq(&entry) {
        inchars.push((' ', 5));
        return Ok(inchars);
    }

    // split them in chars
    let echars: Vec<char> = entry.chars().collect();
    let mut achars: Vec<char> = answer.chars().collect();
    // let achars_length  = achars.len();

    // initialize the inchars with the entry.
    for ae in &echars {
        inchars.push((*ae, 0));
    }

    // inefficiently compare each char in entry.
    // 0 not in answer
    // 1 correct position
    // 2 in word but incorrect position.
    // TODO: If the letter is a 1 or 2 make sure you handle a character
    // appearing twice.

    //   let mut count = 0;
    // DO a loop through the entry and mark, then remove letters
    // in the entry.
    for (count, e) in echars.iter().enumerate() {
        // println!("{}",e);
        if e == &achars[count] {
            // println!("This char is in correct position {}", e);
            inchars[count] = (*e, 1);
            achars[count] = ' ';
        }
        // println!("achars {:?}", achars);

        // count += 1;
    }

    for e in &echars {
        // println!("Checking {}", e);
        if achars.contains(e) {
            // println!("Found {}", e);
            let afoundindex = achars.iter().position(|r| r == e).unwrap();
            // let efoundindex = echars.iter().position(|r| r == e).unwrap();

            // println!("{:?}", afoundindex);
            achars[afoundindex] = ' ';
            for item in inchars.iter_mut().take(achars.len()) {
                if item.0 == *e && item.1 == 0 { 
                    item.1 = 2;
                    break;
                }
            }

/*             for i in 0..achars.len() {
                if inchars[i].0 == *e && inchars[i].1 == 0 {
                    // println!("Updating inchars[{}]", i);
                    inchars[i].1 = 2;
                    break;
                }
            } */
        }
    }

    /*     println!("achars: {:?}", achars);
    println!("echars: {:?}", echars);
    println!("inchars: {:?}", inchars); */
    Ok(inchars)
}

// Instructions might be a useful struct.
pub struct Instructions();

impl Instructions {
    fn guess_instructions() -> String {
        String::from("Enter a word of five characters")
    }

    fn general_instructions() -> String {
        let mut ret = String::from(
            "Currently, the return after the guess shows\nthe character in green, yellow or red.\n",
        );
        ret = ret
            + "green "
            + "means the character is in the correct position\n\
            yellow means the character is the word in the wrong position\n\
            red means the character is not in the word";
        let cs = ColoredString::from("");
        println!("{}", cs.blue());
        ret
    }
}

// Words might of been better named dictionary.
pub struct Words(Vec<String>);

impl Words {
    pub fn new() -> Self {
        let text = std::fs::read_to_string("./5letterwords.txt").unwrap();
        let words: Vec<String> = text.trim().lines().map(|v| v.to_string()).collect();
        Self(words)
    }

    pub fn pick_at_random(&self) -> &str {
        // Get a random number.
        let num = rand::thread_rng().gen_range(0..self.0.len());
        // Return a word
        &self.0[num]
    }
    pub fn check_word_is_valid(&self, word: &str) -> bool {
        let ret = &self.0.contains(&word.to_string());
        *ret
    }
}

impl Default for Words {
    fn default() -> Self {
        Self::new()
    }
}
fn get_word() -> String {
    let mut std_input_lock = io::stdin().lock();
    let input_word = &mut String::new();
    loop {
        input_word.clear();
        println!("enter word:");
/*         match std_input_lock.read_line(input_word) {
            Err(e) => println!("{:?}", e),
            _ => (),
        } */
        if let Err(e) = std_input_lock.read_line(input_word) { println!("{:?}", e) }
        let chars: Vec<char> = input_word.chars().collect();
        // TODO: Update to handle a '?' or possibly other characters and phrases.
        if chars.len() != 6 {
            println!("Not 5 letters");
        } else {
            break;
        }
    }
    input_word.to_string().to_lowercase()
}

#[derive(Debug, Clone)]
/// Key represents 
pub struct Key {
    character: char,
    row: u8,
    // pos: u8,
    value: u8,
}

fn print_keys(keys: &Vec<Key>) {
    let mut this_row = 0_u8;
    for key in keys {
        if this_row != key.row {
            println!();
            this_row = key.row;
        }
        match key.value {
            4 => print!("{} ", key.character.to_string().red().on_red()),
            5 => print!("{} ", key.character.to_string().green()),
            6 => print!("{} ", key.character.to_string().yellow()),
            _ => print!("{} ", key.character.to_string().blue()),
        }
    }
    println!();
}

fn print_entry(keys: &Vec<(char, i32)>) {
    for key in keys {
        match key.1 {
            0 => print!("{} ", key.0.to_string().red()),
            1 => print!("{} ", key.0.to_string().green()),
            2 => print!("{} ", key.0.to_string().yellow()),
            _ => print!("{} ", key.0.to_string().blue()),
        }
    }
    /*     for i in 0..keys.len() {
        match keys[i].1 {
            0 => print!("{} ", keys[i].0.to_string().red()),
            1 => print!("{} ", keys[i].0.to_string().green()),
            2 => print!("{} ", keys[i].0.to_string().yellow()),
            _ => print!("{} ", keys[i].0.to_string().blue()),
        }
    } */
    println!();
}

fn update_keys(current_entry: &Vec<(char, i32)>, keyboard: Vec<Key>) -> Vec<Key> {
    // update keyboard based on current_entry
    // for each enty check keyboard for a 0,1, or 2.
    let mut ret = keyboard;
    // let mut character = ' ';
    // let mut letter_value = 0 as u8;

    for item in current_entry {
        let character = item.0;
        let mut letter_value = item.1;
        if letter_value == 0 {
            letter_value = 4;
        }
        if letter_value == 1 {
            letter_value = 5;
        }
        if letter_value == 2 {
            letter_value = 6;
        }
        for i in &mut ret {
            if i.character == character {
                i.value = letter_value as u8;
                break;
            }
        }
    }
    ret

}
fn create_keys() -> Vec<Key> {
    let mut keys: Vec<Key> = Vec::new();
    let top_keys = "qwertyuiop";
    let mid_keys = "asdfghjkl";
    let bot_keys = "zxcvbnm";
    // let mut count = 0;
    for key_name in top_keys.chars() {
        keys.push(Key {
            character: key_name,
            row: 0,
            value: 0,
        });
    }
    for key_name in mid_keys.chars() {
        keys.push(Key {
            character: key_name,
            row: 1,
            value: 0,
        });
    }
    for key_name in bot_keys.chars() {
        keys.push(Key {
            character: key_name,
            row: 2,
            value: 0,
        });
    }

    keys
}

pub fn run() -> String {
    let mut keyboard = create_keys();
    print_keys(&keyboard);

    // println!("{:?}", create_keys());

    let mut grid: Vec<String> = Vec::new();

    // Implement the Words public structure to get the answer.
    let binding = Words::new();
    let answer = Words::pick_at_random(&binding);
    // let answer = "needs";

    if answer.is_empty() {
        return "".to_string();
    }
    grid.push(answer.to_string());
    // We need to create a loop and when the return is answer = entry win
    // Otherwise give an amount of guesses.

    println!("{}", Instructions::general_instructions());
    for i in 0..6 {
        println!("Guess #:{}", i + 1);
        println!("{}", Instructions::guess_instructions());
        let mut entry = get_word();
        loop {
            // For some reason randomword returns 6 characters.  Probably a \n.
            // It won't match if the \n is present.
            entry = entry[0..entry.len() - 1].to_string();
            if Words::check_word_is_valid(&binding, &entry) {
                break;
            }
            println!(
                "{} not in dictionary! Please try again! CTRL+C to quit.",
                entry
            );
            entry = get_word();
        }

        // If there is already a wrong answer, pop it off the stack.
        if grid.len() != 1 {
            grid.pop();
        }
        grid.push(entry.clone());
        let this_entry = check_answer(grid.clone()).unwrap();
        keyboard = update_keys(&this_entry, keyboard.clone());
        print_keys(&keyboard);
        print_entry(&this_entry);
        // println!("Length of answer:{} entry:{}", answer.len(), entry.len());
        if answer.eq(&entry) {
            println!("Match!");
            break;
        } /* else {
              println!("No Match! the word was {:?}", answer);
          } */
        // println!("Words: {} {}", answer, entry);
    }

    // println!("{:?}",grid);

    answer.to_string()
}
