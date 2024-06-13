pub fn for_iter_1a<T: std::fmt::Display>(v: &Vec<T>) {
    for x in v {
        println!("{x}");
    }
}

pub fn for_iter_1b<T: std::fmt::Display>(v: &Vec<T>) {
    for x in v.iter() {
        println!("{x}");
    }
}

pub fn main() {
    use std::env;
    use std::iter::Iterator;

    let mut args = env::args();
    args.next(); // skip args[0]

    while let Some(arg) = args.next() {
        let v = arg.chars().collect();
        for_iter_1a(&v);
        for_iter_1b(&v);
    }
}
