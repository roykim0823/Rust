fn main() {
    declare_variable();     println!();
    integer_date_type();    println!();
    float_data_type();      println!();
    arithmetic();           println!();
    foramtting_print();     println!();
    bitwise();              println!();
    boolean();              println!();
    comparison();           println!();
    char();                 println!();
}

fn declare_variable() {
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);
}

fn integer_date_type() {
    let mut x: u8 = 255;
    // x = x + 1;  // run time error: attemp to add with overflow
    x = x - 1;  // 
    println!("x is {}", x);
}

fn float_data_type() {
    let x: f32 = 10.123456789123456789;
    println!("x is {}", x);
}

fn arithmetic() {
    let a = 10;
    let b = 3.0;
    let c = a as f64 / (b + 1.0);
    println!("c is {}", c);
}

fn foramtting_print() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    print!("c is {0:08.3}\na is {1}\nonce again, c is {0}\nlastly, c is {0:9.3}\n", c, a);
}

fn bitwise() {
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = value; // NOT
    println!("value is {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    println!("value is {:08b}", value);

    value = value << 4; // shift left by 4
    println!("value is {:08b}", value);

    value = value >> 2; // shift left by 2
    println!("value is {:08b}", value);
}

fn boolean() {
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    let c = (a ^ b) || panic!();  // short circuiting: panic is skipped
    println!("c is {}", c);
}

fn comparison() {
    let a = 1;
    let b: i32 = 2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
}

fn char() {
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);
}