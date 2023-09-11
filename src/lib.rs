//use std::fs;
use std::fs::File;
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


fn myread(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;

    let mut data = String::new();
    file.read_to_string(&mut data).map(|_| data)
}

fn randomword() -> String {
    let filename = "./5letterwords.txt";

    // let list = myread(filename).unwrap();
    let mut list = "".to_string();
    match myread(filename) {
        Ok(contents) => list = contents,
        Err(err) => println!("Unable to open file {}! Error:{}", filename, err),
    }
    //let list = fs::read_to_string(filename).unwrap();

    let searchlist: Vec<&str> = list.trim().split("\n").collect();

    println!("{:?}", searchlist.len());
    if searchlist.len() == 1 {
        println!("File {} doesn't exist!", filename)
    }
    let num = rand::thread_rng().gen_range(0..searchlist.len());
    let theword = searchlist[num];

    //    println!("Random word: {:?}", theword);
    //    println!("Random number: {:?}", num);
    theword.to_string()
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

/* fn printcheck(printdata: Vec<(char,i32)>) {
    for i in 0..4 {
        println!(printdata[i]);
    }
} */

pub fn run() -> String {
    let mut grid: Vec<String> = Vec::new();
    let answer = randomword();
    if answer.len() == 0 {
        return "".to_string();
    }
    grid.push(answer.clone());
    // We need to create a loop and when the return is anser = entry win
    // Otherwise give an amount of guesses.

    for i in 0..6 {
        println!("Guess #:{}", i+1);
        let entry6 = getword();

        // For some reason randomword returns 6 characters.  Probably a \n.
        // It won't match if the \n is present.
        let entry: String = entry6[0..entry6.len() - 1].to_string();

        // If there is already a wrong answer, pop it off the stack.
        if grid.len() != 1 {
            grid.pop();
        }
        grid.push(entry.clone());
        println!("{:?}", checkanswer(grid.clone()).unwrap());
        println!("Length of answer:{} entry:{}", answer.len(), entry.len());
        if answer.eq(&entry) {
            println!("Match!");
            break;
        } /* else {
            println!("No Match! the word was {:?}", answer);
        } */
        // println!("Words: {} {}", answer, entry);
    }

    // println!("{:?}",grid);

    answer
}
