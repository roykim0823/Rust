fn main() {
    conditional_expression();   println!();
    multi();    println!();
    conditional_assignment();   println!();
    loops();    println!();
    while_loop();   println!();
    for_loop();     println!();
    multi_loop();
}

fn conditional_expression() {
    let x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }
}

fn multi() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }
}

fn conditional_assignment() {
    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};
    
    /* if make_x_odd {
        x = 1;
    } else {
        // x = 2;
    } */
    
    println!("x is {}", x);    
}

fn loops() {
    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count * 10;  // this will be returned
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
}

fn while_loop() {
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}

fn for_loop() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {  // same as in message.iter()
        println!("item: {}", item);
    }

    for (index, item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        //if item == 'e' {  // compile error: &char == char
        if *item == 'e' {    
             break;
        }
    }

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }
}

fn multi_loop() {
    let mut matrix = [[1, 2, 3],
                      [4 ,5 ,6],
                      [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
           *num += 10;
           print!("{}\t", num);
        }
        println!();
    }

    for row in matrix {
        for num in row {
           print!("{}\t", num);
        }
        println!();
    }
}