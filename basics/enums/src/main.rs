fn main() {
    define_enums();     println!();
    match_operator();   println!();
    enum_methods();     println!();
    option_enum();      println!();
    match_option();     println!();
    if_let();           println!();
    exercise();
}

#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, c
}

fn define_enums() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c)
    }
}

fn match_operator() {
    let my_number = 3u8;

    let result = match my_number {
        0 => "zero",
        // _ => "one",  // match all the rests
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn enum_methods() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}

// enum Option<T> {
//     Some(T),
//     None
// }
fn option_enum() {
    let countdown = [5, 4, 3, 2, 1];
    // let number = countdown[5];  // compile error, index out of bound
    let number = countdown.get(5);  // .get(5) returns None
    println!("number is {:?}", number);
    let number = countdown.get(0);
    let number = number.unwrap_or(&0) + 1;  // unwarp enum Option<T>
    println!("number is {:?}", number);
}

fn match_option() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(2);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
}

fn if_let () {
    let number = Some(13);

    match number {
        Some(13) => print!("thirteen"),
        _ => ()
    }

    // same as above
    if let Some(13) = number {
        print!("thirteen");
    }
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64) // latitude, longitude
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown Location"),
            Location::Anonymous => println!("Anonymous Location"),
            Location::Known(lat, lon) => println!("{}, {}", lat, lon)
        }
    } 
}

fn exercise() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}