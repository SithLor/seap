use stringy::modules::vowels::{count_lowercase_vowels, count_uppercase_vowels};
fn main() {
    let string = "Hello, world!";
    let e: usize = count_lowercase_vowels(string);
    println!("Hello, world! {}", e);
}
