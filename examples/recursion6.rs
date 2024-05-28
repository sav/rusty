// recursion6.rs,
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Recursion to Iteration: Recursive Data Structures
//!
//! A summary of this great article by [Tom Moertel](https://x.com/tmoertel):
//!    - [Tricks of the trade: Recursion to Iteration, Part 3: Recursive Data
//!      Structures](https://blog.moertel.com/posts/2013-06-03-recursion-to-iteration-3.html)

#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
struct BinaryTree {
    value: i128,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

fn flatten_v0(bst: Option<Box<BinaryTree>>) -> Vec<i128> {
    match bst {
        None => Vec::<i128>::new(),
        Some(tree) => {
            let mut left = flatten_v0(tree.left);
            left.push(tree.value);
            left.extend(flatten_v0(tree.right));
            left
        }
    }
}

fn flatten(bst: Option<Box<BinaryTree>>) -> Vec<i128> {
    let mut stack = Vec::new();
    let mut result = Vec::new();
    let mut current = bst;

    while current.is_some() || !stack.is_empty() {
        // Traverse to the leftmost node
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.left;
        }
        // Current node is None, so we pop from the stack
        if let Some(node) = stack.pop() {
            result.push(node.value);
            current = node.right;
        }
    }

    result
}

fn main() {
    let tree0 = BinaryTree {
        value: 1,
        left: None,
        right: None,
    };
    let tree1 = BinaryTree {
        value: 5,
        left: None,
        right: None,
    };
    let tree2 = BinaryTree {
        value: 7,
        left: Some(Box::new(tree0.clone())),
        right: Some(Box::new(tree1.clone())),
    };

    println!(
        "flatten_v0 = {:?}",
        flatten_v0(Some(Box::new(tree2.clone())))
    );

    println!(
        "flatten = {:?}",
        flatten(Some(Box::new(tree2.clone())))
    );
}
