// never.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

use std::rc::Rc;

fn main() {
    println!("{:?}", std::mem::size_of::<()>());
    println!("{:?}", std::mem::size_of::<&()>());
    println!("{:?}", &().clone());
    println!("{:?}", () == ().clone());
    println!("{:?}", &() == &().clone());

    let c = Rc::new(());
    println!("{:?}", c);
    println!("{:?}", Rc::clone(&c));
    println!("{:?}", c.clone());
    
}
