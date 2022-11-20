#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct SomeNum {
    num: i32,
}

// print the struct above
fn print_some_struct(the_struct: &SomeNum){
    println!("{:?}", the_struct);
}

// mutable the struct with &mut
fn mutable_struct(the_struct: &mut SomeNum) {
    the_struct.num = 5
}

fn main() {
    let mut some_struct: SomeNum = SomeNum{num: 3};
    print_some_struct(&some_struct);
    mutable_struct(&mut some_struct);
    print_some_struct(&some_struct);
}

// notes:
// everything in rust is read-only