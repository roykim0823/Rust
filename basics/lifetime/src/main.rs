fn main() {
    borrow_checker();   println!();
    borrow_checker_fixed();     println!();
    lifetime_annotation();      println!();
    lifetime_annotation2();     println!();
    lifetime_elision();         println!();
    struct_lifetime_annotation();
}

fn borrow_checker() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }
    // println!("propellant is {}", propellant);  // compiler error, &rp1 is invalid
}

fn borrow_checker_fixed() {
    let propellant;
    let rp1 = String::from("RP-1");
    {
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }
    println!("propellant is {}", propellant);  // compiler error, &rp1 is invalid
}

fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_annotation() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    println!("result is {}", result);
}

fn best_fuel2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        // y  // it is not possible return different lifetime
        x
    }
}

fn lifetime_annotation2() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel2(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}

fn lifetime_elision() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

// fn get_first_word<'a>(s: &'a str) -> &'a str {
fn get_first_word(s: &str) -> &str {  // same as above by lifetime elision rules    
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }
    &s // no spaces found; input is a single word
}

struct Shuttle {
    name: String
}

impl Shuttle {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting message: {}", msg);
        &self.name
    }
}

struct Shuttle_str<'a> {
    name: &'a str
}

impl<'a, 'b> Shuttle_str<'a> {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting message: {}", msg);
        self.name
    }

    // use lifetime annotation to return msg
    fn send_transmission_msg(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn struct_lifetime_annotation() {
    let vehicle = Shuttle {
        name: String::from("Endeavour")
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);

    let vehicle = Shuttle_str {
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
    let sender = vehicle.send_transmission_msg("Greetings from orbit!");
    println!("sender is {}", sender);



}