// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($e:expr) => {
        println!("Look at this other macro: {}", $e);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
