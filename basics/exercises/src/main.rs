fn main() {
    //avg();
    //celsius();
}

#[test]
fn avg() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* YOUR CODE GOES HERE */
    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}

#[test]
fn celsius() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0  // expression returns a value and does not ends with ';'
}

#[test]
fn max_min_mean() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter() {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
        mean += num as f64;
    }
    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}

#[test]
fn trim_space() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " x🚀xx ";
    assert_eq!(trim_spaces(test7), "x🚀xx");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    // locate the first non-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }
    
    // search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}

struct Rectangle{
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}

#[test]
fn shape() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}

// fn sum_boxes<T>(a: Box<T>, b: Box<T>) -> Box<T> {
// Add Add Traits below
fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}

#[test]
fn test_box() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(sum_boxes(one, two), Box::new(3));
    
    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);
    
    println!("Tests passed!");
}