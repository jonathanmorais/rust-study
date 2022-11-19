#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let s1: String = gives_ownership(); // move the ownership to variable
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2); // move the ownership from s2 to s3 with a fn
    println!("s1 {}, s3 {}", s1, s3)
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");

    some_string
} 

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}