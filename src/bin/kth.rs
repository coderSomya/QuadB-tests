use std::io::{self, Write};

fn main(){
    let mut input= String::new();
    let mut k_input= String::new();

    println!("Enter the numbers seperated by whitespace");
    io::stdin().read_line(& mut input).expect("Failed to read input array");

    let arr: Vec<u32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please input valid integers")).collect();

    io::stdout().flush();
    println!("Enter k");
    io::stdin().read_line(&mut k_input).expect("Failed to read input k");

    let k: usize = k_input.trim().parse().expect("Please input a valid integer");

    if k==0 || k>arr.len(){
        println!("Invalid value of k");
    }

    else {
        let result = kth_smallest(arr, k);
        println!("Kth smallest value in the array is {result}");
    }
}

fn kth_smallest(mut arr: Vec<u32>, k: usize)-> u32{
    arr.sort();
    arr[k-1]
}