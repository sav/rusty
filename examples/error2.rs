use std::fs::File;
use std::io;

fn handle_file(path: &str) {
    let f: io::Result<File> = File::open(path);

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => match File::create(path) {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Failed creating file: {}: {:?}", path, e)
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {}: ${:?}",
                path, error
            )
        }
    };

    println!("{:?}\n~~~ ~~~ ~~~\n", f);
}

fn main() {
    handle_file("/tmp/hello.txt");
    handle_file("/dev/hello.txt");
}
