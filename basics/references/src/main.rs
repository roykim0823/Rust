fn main() {
    return_by_move();       println!();
    borrowing_reference();  println!();
    mutable_reference();    println!();
    dangling_ref();         println!();
    slices();               println!();
    slices_function_param();println!();
}

fn return_by_move() {  // naive approach
    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel, length) = process_fuel(rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: String) -> (String, usize) {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    (propellant, length)
}

fn borrowing_reference() {
    let rocket_fuel = String::from("RP-1");
    let length = process_fuel_ref(&rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel_ref(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    //let length = propellant.len();
    //length
    propellant.len()
}

fn mutable_reference() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel_mut(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel_mut(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

fn dangling_ref() {
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel is {}", rocket_fuel);
}

// fn produce_fuel() -> &String {
//     let new_fuel = String::from("RP-1");
//     &new_fuel  // compile error, dangling reference
// }

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel  // move
}

fn slices() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word = &message[15..];
    println!("last_word is {}", last_word);
    
    let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);
}

fn slices_function_param() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
    // let first_word = get_first_word(&message[10..]);  // &str => &String (X)
    let first_word = get_first_word_str(&message[10..]);  // &str => &String (X)
    println!("first_word is {}", first_word);
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}

fn get_first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}