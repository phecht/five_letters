//use std::fs;
// use std::fs::File;
use std::io;
use std::io::prelude::*;

// use std::error::Error;
use rand::Rng;

fn checkanswer(mut grid: Vec<String>) -> Result<Vec<(char, i32)>, std::io::Error> {
    // pop the answer and entry words off th Vec.
    let entry = grid.pop().unwrap();
    let answer: String = grid.pop().unwrap();
    //let answer: String = grid[i].unwrap();

    // define a Vec with a (char, i32) that will hold the return
    let mut inchars: Vec<(char, i32)> = Vec::new();
    // let mut count_chars: Vec<(char,i32)> = Vec::new();

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

    let mut count = 0;
    for e in echars {
        // println!("{}",e);
        if e == achars[count] {
            // println!("This char is in correct position {}", e);
            inchars[count] = (e, 1);
            achars[count] = ' ';
        } else if achars.contains(&e) {
            // println!("{} letter is not in proper place", e);
            inchars[count] = (e, 2);

            let in_word_char = e;
            // println!("in_word_char {}",in_word_char) ;
            for i in 0..achars.len() {
                // println!("{:?}", achars);
                if in_word_char == achars[i] {
                    achars[i] = ' ';
                    break;
                }
            }
        }
        // println!("achars {:?}", achars);

        count = count + 1;
    }
    //println!("{:?}",inchars);
    Ok(inchars)
}

// Instructions might be a useful struct.
pub struct Instructions();

impl Instructions {
    fn guess_instructions() -> String {

        let ret  = String::from("Enter a word of five characters");
        ret
    }

    fn general_instructions() -> String {

        let mut ret  = String::from("Currently, the return after the guess shows\nthe character and a 1,2 or zero.\n");
        ret = ret + "1 means the character is in the correct position\n\
            2 means the character is the word in the wrong position\n\
            0 means the character is not in the word";
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

fn get_word() -> String {
    let mut std_input_lock = io::stdin().lock();
    let input_word = &mut String::new();
    loop {
        input_word.clear();
        println!("{}", "enter word:");
        match std_input_lock.read_line(input_word) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
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

pub fn run() -> String {

    let mut grid: Vec<String> = Vec::new();

    // Implement the Words public structure to get the answer.
    let binding = Words::new();
    let answer = Words::pick_at_random(&binding);

    if answer.len() == 0 {
        return "".to_string();
    }
    grid.push(answer.to_string());
    // We need to create a loop and when the return is answer = entry win
    // Otherwise give an amount of guesses.

    println!("{}",Instructions::general_instructions());
    for i in 0..6 {
        println!("Guess #:{}", i + 1);
        println!("{}",Instructions::guess_instructions());
        let mut entry = get_word();
        loop {
        // For some reason randomword returns 6 characters.  Probably a \n.
        // It won't match if the \n is present.
            entry = entry[0..entry.len() - 1].to_string();
            if Words::check_word_is_valid(&binding, &entry) {
                
                break;
            }
            println!("{} not in dictionary! Please try again! CTRL+C to quit.",entry);
            entry = get_word();
        }



        // If there is already a wrong answer, pop it off the stack.
        if grid.len() != 1 {
            grid.pop();
        }
        grid.push(entry.clone());
        println!("{:?}", checkanswer(grid.clone()).unwrap());
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
