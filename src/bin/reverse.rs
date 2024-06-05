use std::io::{self, Write};

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    println!("Enter a string:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}
