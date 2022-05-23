/***************************
 * my_project binary crate *
 ***************************/

mod some_module;
use package_crates; // library crate for package_crates package

fn main() {
    println!("Running the my_project executable.");
    some_module::mod_func();
    package_crates::lib_func();
}

// build all: cargo build --workspace