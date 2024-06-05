use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <string>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    if is_palindrome(input) {
        println!("The string is a palindrome.");
    } else {
        println!("The string is not a palindrome.");
    }
}

fn is_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let len = s.len();
    for i in 0..len / 2 {
        if s[i] != s[len - 1 - i] {
            return false;
        }
    }
    true
}
