#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // shadowing
    let age: &str = "47"; // immutable
    let mut age: u32 = age.trim().parse().expect("age wasn't assigned a number"); // mutable / parse to int 32
    
    age = age + 1; // mutabled
    println!("I'm {} and I want ${}", age, ONE_MIL)


}