#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn arrays_setup() {
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_index: usize = 0;

    loop {
        if arr_1[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr_1[loop_index] == 9 {
            break;
        }

        println!("Array value: {}", arr_1[loop_index]);
        loop_index += 1;
    }
}    

fn while_loop() {
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_index: usize = 0;

    while loop_index < arr_1.len() {
        println!("Arr: {}", arr_1[loop_index]);
        loop_index += 1;
    }
}

fn main() {
    while_loop();
}
