fn main() {
    array();    println!();
    multi();    println!();
    tuples();   println!();
}

fn array() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index-1]);
}

fn multi() {
    let parking_lot = [[1, 2, 3],
                       [4, 5, 6]];

    let number = parking_lot[1][2];
    println!("number is {}", number);
    
    let garage = [[[0; 10]; 10]; 100];
    println!("garage is {}", garage.len());
    println!("garage[0] is {}", garage[0].len());
    println!("garage[0][0] is {}", garage[0][0].len());
}

fn tuples() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("(a, b, c) is ({0}, {1}, {2})", a, b, c);
}