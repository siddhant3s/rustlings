// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
        struct Sid {a: String, b: String};
        42
    };
}


fn main() {


    my_macro!();
    let sid = Sid {a: String::from(""), b: String::from("")};

}
