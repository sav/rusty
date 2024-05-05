// thread1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

use std::thread;

fn ex1_map_reduce() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];
    let data = data.split_whitespace();

    for (i, segment) in data.enumerate() {
        println!("segment #{i} is: {segment}");
        children.push(thread::spawn(move || -> u32 {
            let result = segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment #{i}, result={result}");
            result
        }));
    }

    let result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    println!("final result: {result}");
}

fn main() {
    println!("-=- ex1_map_reduce() -=-");
    ex1_map_reduce();
}
