/// Take a word and see if it matches a random word.
/// If it doesn't match, show which letters are in correct position
/// and letters that are in the word but isn't in the right position.
use std::io;
use std::io::prelude::*;

fn main() {

    let the_word = getword();

    println!("The word is {}",the_word);

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
        i1.to_string()
    }

}
