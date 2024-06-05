use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

// Define a TreeNode structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

// Function to add edge to the tree
fn add_edge(parent: &mut Option<Rc<RefCell<TreeNode>>>, child_val: i32, is_left: bool) {
    let child = TreeNode::new(child_val);

    if is_left {
        parent.as_ref().unwrap().borrow_mut().left = Some(child.clone());
    } else {
        parent.as_ref().unwrap().borrow_mut().right = Some(child.clone());
    }
}

// Function to calculate the maximum depth of the binary tree
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter edges in the format 'parent child L/R' (L for left, R for right). Enter 'done' when finished:");

    // Read the input from the user
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;
    loop {
        print!("Edge: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input == "done" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Invalid input format. Try again.");
            continue;
        }

        let parent_val: i32 = parts[0].parse().expect("Invalid integer value");
        let child_val: i32 = parts[1].parse().expect("Invalid integer value");
        let is_left = match parts[2] {
            "L" | "l" => true,
            "R" | "r" => false,
            _ => {
                println!("Invalid direction. Use 'L' for left and 'R' for right. Try again.");
                continue;
            }
        };

        if root.is_none() {
            root = Some(TreeNode::new(parent_val));
        }

        add_edge(&mut root, child_val, is_left);
    }

    // Calculate and print the maximum depth of the binary tree
    let depth = max_depth(root);
    println!("The maximum depth of the binary tree is: {}", depth);
}
