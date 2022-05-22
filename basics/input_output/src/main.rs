fn main() {
    cmd_line(); println!();
    read_file();    println!();
    write_file();   println!();
    check_roster();
}

use std::env;

fn cmd_line() {
    if env::args().len() <= 2{
        println!("Program requires as least 2 arguments.");
        return;
    }
    
    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}

use std::fs;
use std::io::prelude::*;

fn read_file() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents);
}

fn write_file() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");
}

fn check_roster() {
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);
}