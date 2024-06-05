use std::io::{self, Write};

fn main() {
    print!("Enter a list of integers separated by spaces: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();


    let max_sum = max_subarray_sum(&nums);
    
    println!("The maximum subarray sum is: {}", max_sum);
}

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_current = nums[0];
    let mut max_global = nums[0];
    
    for &num in nums.iter().skip(1) {
        max_current = i32::max(num, max_current + num);
        if max_current > max_global {
            max_global = max_current;
        }
    }
    
    max_global
}
