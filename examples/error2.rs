use std::fs::File;
use std::io;

pub fn handle_file(path: &str) {
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

pub fn handle_file_alt() {
    // a construct similar to the above but does not contain any `match`,
    // and thus it is cleaner to read.
    let greeting_file = File::open("/etc/motd").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("motd").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", greeting_file);
}

pub fn handle_file_expect(path: &str) {
    let f = File::open(path)
        .expect("file not accessible: {path}");
    println!("file is accessible: {f:?}");
}

pub fn main() {
    handle_file("/tmp/hello.txt");
    handle_file_alt();
    handle_file_expect("/etc/motd");
    handle_file("/dev/hello.txt"); // PermissionDenied
}
