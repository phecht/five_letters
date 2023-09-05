//use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

// use std::error::Error;
use rand::Rng;

fn checkanswer(mut grid: Vec<String>) -> Result<Vec<(char,i32)>,std::io::Error> {
    // pop the answer and entry words off th Vec.
    let entry = grid.pop().unwrap();
    let answer: String = grid.pop().unwrap();

    // split them in chars
    let echars: Vec<char> = entry.chars().collect();
    let achars: Vec<char> = answer.chars().collect();

    // define a Vec with a (char, i32) that will hold the return
    let mut inchars: Vec<(char,i32)> = Vec::new();

    // initialize the inchars with the entry.
    for ae in &echars {
        inchars.push((*ae,0));
        
    }

    // inefficiently compare each char in entry.
    // 0 not in answer
    // 1 correct position
    // 2 in word but incorrect position.  
    let mut count = 0;
    for e in echars {
        // println!("{}",e);
        if e == achars[count] {
            // println!("This char is in correct position {}", e);
            inchars[count] = (e,1);
            
        } else if achars.contains(&e) {
            // println!("{} letter is not in proper place", e);
            inchars[count] = (e,2);
        }
        count = count + 1;
    }
    //println!("{:?}",inchars);
    Ok(inchars)
}

// }

fn myread(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;

    let mut data = String::new();
    file.read_to_string(&mut data).map(|_| data)
}
// -> Result<String, Box<dyn Error>>
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

    println!("Random word: {:?}", theword);
    println!("Random number: {:?}", num);
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
    i1.to_string()
}

pub fn run() -> String {
    let answer = randomword();
    if answer.len() == 0 {
        return "".to_string();
    }
    let entry6 = getword();

    // For some reason randomword returns 6 characters.  Probably a \n.
    // It won't match if the \n is present.
    let entry: &str = &entry6[0..entry6.len() - 1];

    let mut grid: Vec<String> = Vec::new();
    grid.push(answer.clone());
    grid.push(entry.clone().to_string());
    println!("{:?}",checkanswer(grid).unwrap());

    println!("Length of answer:{} entry:{}", answer.len(), entry.len());
    if answer.eq(&entry) {
        println!("Match!");
    } else {
        println!("No Match!");
    }
    println!("Words: {} {}", answer, entry);
    answer
}
