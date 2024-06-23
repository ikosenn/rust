use std::fs::File;
use std::io::ErrorKind;


pub fn read_or_create(filename: &str) {
    let file_ptr = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creatin file: {error:?}");
            })
        } else {
            panic!("Problem opening file: {error:?}");
        }
    });
}
