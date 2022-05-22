fn main() {
    scope();    println!();
    shadowing();    println!();
    string();   println!();
    move_copy_clone();  println!();
    move_copy_clone2();  println!();
    move_copy_clone3();  println!();
    transfer();     println!();
    transfer2();    println!();
}

fn scope() {
    let planet = "Earth";
    if true {   
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);
}

fn shadowing() {
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);
}

fn string() {
    let mut message = String::from("Earth");
    println!("message is {}", message);
    message.push_str(" is home.");
    println!("message is {}", message);
}

fn move_copy_clone() {
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet;  // inner_plant is moved
        // println!("inner_planet is {}", inner_planet);  // invalid
    }
    println!("outer_planet is {}", outer_planet);
}

fn move_copy_clone2() {
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet.clone();  // deep copy
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}

fn move_copy_clone3() {
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;  // copy on builtin-type (stack-only data)
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}

fn transfer() {
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);  // argument is copied
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(mut propellant: i32) {
    propellant += 1;
    println!("processing propellant {}...", propellant);
}

fn transfer2() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel2(rocket_fuel);  // argument is moved
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel2(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}