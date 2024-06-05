use std::io::{self, Write};

fn merge_sorted_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

fn main() {
    let mut input1 = String::new();
    println!("Enter the first sorted array of integers separated by spaces:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let arr1: Vec<i32> = input1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let mut input2 = String::new();
    println!("Enter the second sorted array of integers separated by spaces:");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let arr2: Vec<i32> = input2
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let resultant = merge_sorted_arrays(arr1, arr2);
    let len = resultant.len();

    println!("The merged array is: ");
    for i in 0..len{
        let a = resultant[i];
        print!("{a} ");
    }
}

