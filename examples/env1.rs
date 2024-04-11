use std::env;

extern crate env_logger;

pub fn main() {
    env_logger::init();
    let name = env::args().skip(1).next();
    println!("Hello, {}!", name.unwrap_or("world".into()));
}
