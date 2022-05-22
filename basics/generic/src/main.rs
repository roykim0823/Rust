fn main() {
    generic_struct();   println!();
    println!("biggest is {}", get_biggest(1.2, 2.3));   println!();
    box_data();     println!();
}

#[derive(Debug)]    
struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {  // Specialization?
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}


fn generic_struct() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());

    let rect_T = Rectangle {
        width: 1u8,
        height: 3u8
    };
    println!("perimeter is {}", rect_T.get_perimeter());
}

// Use std::cmp::PartialOrd Trait
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

use std::mem;

struct Shuttle {        // 40 bytes = 
    name: String,       // 8 (ptr) + 8 (len) + 8 (cap) = 24 bytes
    crew_size: u64,     // 8 bytes (due to alignment, u8 ~ u64: 8 bytes)
    propellant: f64     // 8 bytes
}

fn box_data() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    // box is a smart pointer which store data on a heap instaed of on a stack
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
}
