 use std::io;
 use std::io::prelude::*;
 
 fn main() {
    let input = io::stdin();
    let lines = input 
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    for x in lines.iter(){
        let number_pairs = x
            .split(" ")
            .map(|component| component.parse::<u64>().unwrap()) 
            .collect::<Vec<u64>>(); 

        if number_pairs[0] > number_pairs[1] {
            let diff = number_pairs[0] - number_pairs[1];
            println!("{}", diff);
        }
        else {
            let diff = number_pairs[1] - number_pairs[0];
            println!("{}", diff);
        }
    }
 }