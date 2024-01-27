use std::fs::File;

fn handle_file(path: &str) {
    let f = File::open(path).unwrap();
    println!("{:?}\n~~~ ~~~ ~~~\n", f);

    let f = File::open(path).expect("Failed to open file");
    println!("{:?}\n~~~ ~~~ ~~~\n", f);
}

fn main() {
    handle_file("/tmp");
    handle_file("/tmp/hello.txt");
}
