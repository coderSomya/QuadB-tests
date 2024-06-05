use std::io;

fn main(){
    let mut input = String::new();

    println!("Enter a sorted array of integers separated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input.trim().split_whitespace()
                             .map(|s| s.parse().expect("Please enter valid integers"))
                             .collect();

    println!("Enter the target number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Please enter a valid integer");

    match binary_search(&arr, target){
        Some(index)=> println!("The first occurence of {target} is at {index} index"),
        None => println!("The element is not found in the array")
    }
}


fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] == target {
            result = Some(mid as usize);
            right = mid - 1;
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}