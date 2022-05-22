fn main() {
    traits();   println!();
    derive_traits();    println!();
    trait_bound();      println!();
    multiple_trait_bound();      println!();
    return_trait();     println!();
    implement_display_trait();
}

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

trait Description {  // similar to interface
    fn describe(&self) -> String;

    // default trait implementation
    fn describe_default(&self) -> String {
        String::from("an object flying through space!")
    }
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!", self.name, self.velocity)
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
    fn describe_default(&self) -> String {
        format!("the {} flying {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
}

fn traits() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
    println!("hubble is {}", hubble.describe_default());
    println!("iss is {}", iss.describe_default());
}

#[derive(PartialEq, PartialOrd)]
struct SatelliteDerived {
    name: String,
    velocity: f64 // miles per second
}

fn derive_traits() {
    let hubble = SatelliteDerived {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let gps = SatelliteDerived {
        name: String::from("GPS"),
        velocity: 2.42
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}

use std::any;
use std::fmt;

// fn print_type<T: fmt::Display>(item: T) {
//    println!("{} is {}", item, any::type_name::<T>());
fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}

fn trait_bound() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);  // requires fmt::Debug
}

// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
    where T: fmt::Display + PartialEq + From<U>,
          U: fmt::Display + PartialEq + Copy
{
        if a == T::from(b) {  // Copy instead of Move for below println!(.., b)
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn multiple_trait_bound() {
    // compare_and_print(1.0, "one");  // compile error
    compare_and_print(1.1, 1);
}

fn get_displayable(choice: bool) -> impl fmt::Display {
    if choice {
        13
    } else {
        // "thirteen"  // Need to use dynamic disptach
        0
    }
}

fn return_trait() {
    println!("output is {}", get_displayable(true));
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} flying at {} miles per hour", self.name, self.velocity)
    }
}

fn implement_display_trait() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}