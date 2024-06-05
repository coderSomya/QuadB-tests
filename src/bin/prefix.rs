use std::io;

fn main(){

    let mut input = String::new();
    println!("Enter a list of strings seperated by whitespace");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let words: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    let result = longest_prefix(&words);
    println!("Longest prefix is '{result}'");
}

fn longest_prefix(words: &[String]) -> String{
    if words.is_empty(){
        return String::new();
    }
    let mut prefix = &words[0][..];

    for s in &words[1..]{
        while !s.starts_with(prefix){
            if prefix.is_empty(){
                return String::new();
            }
            prefix = &prefix[..prefix.len()-1];
        }
    }

    prefix.to_string()
}