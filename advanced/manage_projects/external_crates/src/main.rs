// option 1
// use rand::thread_rng;
// use rand::Rng;

// option 2: bring all items
// use rand::*;

// option 3: import most common items
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let num = rng.gen::<u32>();
    println!("num is {}", num);
}