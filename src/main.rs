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

}

// basic hello world stuff, user input, format strings
fn hello_world() {
    println!("What is your name?");
    let mut name = String::new(); // create a mutable variable with 'let mut' & assign an empty string
    let greeting: &str = "Nice to meet you!"; // create immutable variable with 'let' & assign string with &str

    // read a line from the command line, result is moved in buf, &mut because mutable
    io::stdin().read_line(&mut name) // returns an enum
        .expect("No input"); // catch error

    // println! is a macro, use '{}' to format, .trim() gets rid of the newline character
    println!("Hello, {}! {}", name.trim(), greeting);
}

// shadowing, parsing and constants
fn shadowing() {
    const ONE_MIL: u32 = 1_000_000; // create a constant, pass a type with ':'
    const PI: f32 = 3.141592; // consts with type f32
    let age: &str = "47"; // immutable string

    // mutable u32 with the same name, called shadowing
    let mut age: u32 = age.trim().parse()
        .expect("Age could not be parsed");
    age = age + 1;
    println!("I am {} and i got {}$", age, ONE_MIL);
}

fn datatypes() {
    // rust is statically typed, can be auto generated
    // unsigned integers: u8, u16, u32, u64, u128, usize (64bit on 64bit cpu)
    // signed integer: i8, i16, i32, i64, i128, isize
    // floats: f32, f64
    println!("Max u32: {}", u32::MAX); // get max value with ::MAX
    println!("Max usize: {}", usize::MAX); 
    println!("Max f32: {}", f32::MAX);

    // booleans
    let is_true = false;

    // characters
    let grade = 'a';

}

fn basic_math() {
    let n: f32 = 75.0;
    let m: f32 = 34.0;
    println!("{} + {} = {}", n, m, n + m);
    println!("{} - {} = {}", n, m, n - m);
    println!("{} * {} = {}", n, m, n * m);
    println!("{} / {} = {}", n, m, n / m);
    println!("{} % {} = {}", n, m, n % m);
}