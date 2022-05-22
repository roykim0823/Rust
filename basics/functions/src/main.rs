fn main() {
    function_parameter();   println!();
    function_return();      println!();
}

fn function_parameter() {
    say_hello();
    say_hello();
    let x = 1;  // u8 by compiler's type deduction
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
}

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}

fn function_return() {  // same as () -> () : return unit data type (nothing)
    let result = square(13);
    println!("result is {:?}", result);
}

// fn square(x: i32) -> i32 {
//     x * x  // expression, not statuement
// }

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    // println!("End of function");  // compilation warning: unreachable statement
}