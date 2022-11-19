#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn normal_conditional() {
    let age: i32 = 8;

    if (age >= 1) && (age <= 18) {
        println!("Minor age");
    } else {
        println!("Major age");
    }
}

fn ternary_conditional() {
    let mut age: i32 = 16;
    let can_vote: bool = if age >= 16 {
        true
    } else {
        false
    };

    println!("Can vote: {}", can_vote)

}

fn match_comparisson() {
    let my_age: i32 = 32;
    let vote_age: i32 = 18;

    match my_age.cmp(&vote_age){
        Ordering::Less    => println!("can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal   => println!("you gained right to vote"),
    };
}

fn main() {
    match_comparisson()
}
