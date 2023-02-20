#![allow(unused)] // remove warnings from unused varibles

// create a new project with 'cargo new <name>' or 'cargo init'
// imports packages with 'use'
use std::io; // import the whole namespace
use std::fs::File; // import single item from namespace
use std::cmp::Ordering;
use std::io::{Write, BufRead, BufReader, ErrorKind}; // import specific items from a namespace
use rand::Rng; // search external packages with 'cargo search <name>' and put packages in cargo.toml


// functions are created with 'fn <name>()'
// common intendation level is 4 spaces
fn main() {
    println!("What is your name?");
    let mut name = String::new(); // create a mutable variable with 'let mut' & assign an empty string
    let greeting: &str = "Nice to meet you!"; // create immutable variable with 'let' & assign string with &str

    // read a line from the command line, result is moved in buf, &mut because mutable
    io::stdin().read_line(&mut name) // returns an enum
        .expect("No input"); // catch error

    // println! is a macro, use '{}' to format, .trim() gets rid of the newline character
    println!("Hello, {}! {}", name.trim(), greeting); 
}
