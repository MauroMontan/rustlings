// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

#[macro_use]
pub mod macros {

    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub fn process_temp() {
        println!("hola");
    }
}

fn main() {
    my_macro!();

    macros::process_temp();
}
