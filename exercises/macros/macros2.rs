// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    ($msg:expr) => {
        println!("Check out my macro! Message: {}", $msg);
    };
}

fn main() {
    my_macro!("Hello, macro!");
}