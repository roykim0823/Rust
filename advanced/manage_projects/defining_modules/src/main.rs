fn main() {
    use_hello_module();     println!();
    use_greeting_module();  println!();
}

fn use_hello_module() {
    crate::hello::english(); // absolute path
    crate::hello::spanish(); // hola
    hello::spanish();  // relative path
    crate::hello::casual::english(); // hey
}

mod hello {
    pub fn english() {
        println!("hello");
        spanish();
        casual::english();
    }

    pub fn spanish() {
        println!("hola");
    }

    pub mod casual {
        pub fn english() {
            println!("hey");
            crate::hello::spanish();
            super::spanish();  // relative path: super -> parent module
        }
    }
}

//use crate::greeting::formal;
//use crate::greeting::casual;
use crate::greeting::{formal, casual};

fn use_greeting_module() {
    crate::greeting::formal::english();
    formal::spanish();  // standard convention
    greeting::casual::english();
    casual::spanish();
}

mod greeting {
    pub mod formal {
        pub fn english() {
            println!("hello");
        }

        pub fn spanish() {
            println!("hola");
        }
    }

    pub mod casual {
        pub fn english() {
            println!("hey");
        }

        pub fn spanish() {
            println!("oye");
        }
    }
}