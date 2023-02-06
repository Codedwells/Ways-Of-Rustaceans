//This is a comment that is ignored by rustc
#![allow(unused)]

use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Welcome me Universe!!!");
    println!("I'm a rustacean!!!");

    let age: i32 = 34;

    println!("🔥{}", age);
}

