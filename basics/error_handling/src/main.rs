fn main() {
    unrecoverable();    println!();
    result_enum();      println!();
    match_result();     println!();
    propagate_error();  println!();
    exercise();
}

fn unrecoverable() {
    // panic!("Houston, we've had a problem.");  // unrecoverable

    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println!{"T-minus {}", count};
        // let x = 1 / count; // this won't end well, divide by 0
        let x = 1 / (count+1);
    }
}

use std::fs;

fn result_enum() {
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody know the ultimate question!");
    println!("contents is: {:?}", contents);
}

use std::io;

fn match_result() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error: {:?}", error)
        }
    };

    println!("contents is: {:?}", contents);
}

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?;  // ? is same as below match
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn propagate_error() {
    let result = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is...\n{}", s),
        Err(e) => println!("There was an error: {}", e)
    };
}

use rand::prelude::*;

fn exercise() {
    let secret_number = thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                        Ok(value) => value, // success
                        Err(_) => {
                            println!("\nFailed to parse input. Guess again:");
                            continue
                        }
                     }
            Err(_) => {
                println!("\nFailed to read input. Guess again:");
                continue
            }
        };

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }    
}