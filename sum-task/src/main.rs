use std::io;
use std::io::prelude::*;
 
fn main() {
    let input = io::stdin();
    let mut lines = input 
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
 
    let lenght_of_sequence = lines 
        .next().unwrap()                                   
        .parse::<u32>().unwrap();
 
    let mut amount_of_numbers_to_add;
    if lenght_of_sequence % 2 == 0 {
        amount_of_numbers_to_add = lenght_of_sequence / 2;
    }
    else {
        amount_of_numbers_to_add = (lenght_of_sequence + 1) / 2;
    }
 
    let mut numbers_sequence = lines
        .next().unwrap()
        .trim()
        .split(" ")
        .map(|component| component.parse::<u32>().unwrap())
        .collect::<Vec<u32>>(); 
    numbers_sequence.sort();
    numbers_sequence.reverse(); 
 
    let mut sum: u32 = 0;
    for x in 0..amount_of_numbers_to_add {
        sum += numbers_sequence[x as usize];
    } 
 
    println!("{}", sum);
 }
