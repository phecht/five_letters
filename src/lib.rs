use std::io;
use std::io::prelude::*;
use std::fs;
// use std::error::Error;
use rand::Rng;

// -> Result<String, Box<dyn Error>> 
fn randomword() -> String {
    let list = fs::read_to_string("./5letterwords.txt");

 

    let num = rand::thread_rng().gen_range(1..495);
 
    let newlist = list.unwrap();
    let searchlist = newlist.split("\n");
    let mut counter = 0;
    let mut theword = "".to_string();
    for s in searchlist {
        if counter == num {
            theword = s.to_string();
        }
        counter+=1;
    }
    
    println!("Random word: {:?}", theword);
    println!("Random number: {:?}", num);
    theword

}


/* fn get_name_index(name: &String, array: &Vec<String>) -> Option<usize> {
    array.iter().position(|x| x == name)
}  */

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

pub fn run() -> String {
    let answer = randomword();
    let entry6 = getword();

    // For some reason randomword returns 6 characters.  Probably a \n.
    // It won't match if the \n is present.
    let entry: &str = &entry6[0..entry6.len()-1];
    println!("Length of answer:{} entry:{}", answer.len(),entry.len());
    if answer.eq(&entry) {
        println!("Match!");
    } else {
        println!("No Match!");
    }
    println!("Words: {} {}", answer, entry);
    answer
}