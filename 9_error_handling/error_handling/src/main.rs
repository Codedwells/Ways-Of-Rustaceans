use std::fs::{self,File};
use std::io::{Error, ErrorKind, Read};

fn open_file1() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating creating the file"),
            },
            other_error => {
                panic!("Problem opening the file : {:?}", other_error)
            }
        },
    };
}

// Using closures
fn open_file2() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem while creating your file :{:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;

    //let mut f = match f {
    //    Ok(file) => file,
    //    Err(err) => return Err(err),
    //};

    let mut s = String::new();

    //match f.read_to_string(&mut s) {
    //   Ok(_) => Ok(s),
    //    Err(e) => Err(e),
    //}

    f.read_to_string(&mut s)?;
    Ok(s)
}

// More concise implimentation of read username
fn read_username_from_file2() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Shorter implimentation of read username function
fn read_username_from_file3 () -> Result<String, Error > {
    fs::read_to_string("hello.txt")
}

fn main() {
    open_file2();
}
