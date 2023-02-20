#![allow(unused)] // remove warnings from unused varibles

// create a new project with 'cargo new <name>' or 'cargo init'
// imports packages with 'use'
use std::io; // import the whole namespace
use std::fs::File; // import single item from namespace
use std::cmp::Ordering;
use std::io::{Write, BufRead, BufReader, ErrorKind}; // import specific items from a namespace
use rand::Rng; // search external packages with 'cargo search <name>' and put packages in cargo.toml



fn main() {
    println!("Hello, world!");
}
