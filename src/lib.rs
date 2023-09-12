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


fn getword() -> String {
    let mut s1 = io::stdin().lock();
    let i1 = &mut String::new();
    loop {
        i1.clear();
        println!("{}", "enter word:");
        match s1.read_line(i1) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
        let chars: Vec<char> = i1.chars().collect();
        if chars.len() != 6 {
            println!("Not 5 letters");
        } else {
            break;
        }
    }
    i1.to_string().to_lowercase()
}

pub fn run() -> String {

    let mut grid: Vec<String> = Vec::new();
    //let answer = randomword();
    // let the_words = Words;

    let binding = Words::new();
    let answer = Words::pick_at_random(&binding);

    if answer.len() == 0 {
        return "".to_string();
    }
    grid.push(answer.to_string());
    // We need to create a loop and when the return is anser = entry win
    // Otherwise give an amount of guesses.

    for i in 0..6 {
        println!("Guess #:{}", i + 1);
        let mut entry = getword();
        loop {
                   // For some reason randomword returns 6 characters.  Probably a \n.
        // It won't match if the \n is present.
            entry = entry[0..entry.len() - 1].to_string();
            if Words::check_word_is_valid(&binding, &entry) {
                
                break;
            }
            println!("{} not in dictionary!",entry);
            entry = getword();
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
