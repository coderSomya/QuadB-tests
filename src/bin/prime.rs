use std::io;

fn main(){
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: u32 = input.trim().parse().expect("Please enter a valid number");

    if is_prime(number){
        println!("The given number is prime");
    }
    else{
        println!("The given number is not prime");
    }
}


fn is_prime(num: u32)-> bool{
    if num<=1 {
        return false;
    }
    if num==2 {
        return true;
    }
    if num%2 == 0{
        return false;
    }

    let sqrt_num = (num as f64).sqrt() as u32;

    for i in (3..sqrt_num).step_by(2){
        if num%i==0 {
            return false;
        }
    }
    return true;
}