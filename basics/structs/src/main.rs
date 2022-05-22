fn main() {
    structs();  println!();
    struct_methods();   println!();
    associated_functions(); println!();
    tuple_struct();     println!();
}

#[derive(Debug)]  // Add default traits
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn structs() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("name is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);


    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };
    let vehicle3 = Shuttle {
        // ..vehicle  // compile error since name is moved
        ..vehicle.clone()
    };

    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
    println!("vehicle3 is {:?}", vehicle3);
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 10,
            propellant: 10.0
        }
    }
}

fn struct_methods() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0
    };

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}

fn associated_functions() {
    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}

struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn tuple_struct() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6);
    // let y = get_y(red);  /compile error: wrong type
    let y = get_y(coord);
    println!("y is {}", y);
}