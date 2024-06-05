use std::io;


fn main(){
    let mut input = String::new();

    println!("Enter a sorted array of numbers");
    io::stdin().read_line(&mut input).expect("Failed to read the input");

    let arr: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers"))
    .collect();

    match median(&arr){
        Some(median) => println!("The median of the array is: {median}"),
        None => println!("The array is empty, could not find median")
    }
}

fn median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();

    if len == 0 {
        return None;
    }

    if len%2 == 0{
        let mid1 = len/2 -1;
        let mid2 = len/2;

        Some((arr[mid1]+arr[mid2]) as f64/2.0)
    }
    else{
        Some(arr[len/2] as f64)
    }
}