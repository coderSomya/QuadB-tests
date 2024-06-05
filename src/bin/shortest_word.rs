use std::io;


fn main(){

    let mut input = String::new();

    println!("Enter a string of words:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    match shortest_word(input){
        Some(word)=> println!("The shortest word was '{word}'"),
        None => println!("No words found in the input")
    }
}

fn shortest_word(s: &str) -> Option<&str>{
    s.split_whitespace()
     .min_by_key(|word| word.len())
}