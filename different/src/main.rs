/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

 use std::io;
 use std::io::prelude::*;
 
 /// Kattis calls main function to run your solution.
 fn main() {
    // get standard input stream
    let input = io::stdin();
    // get input lines as strings
    let lines = input 
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    for x in lines.iter(){

        let number_pairs = x
            .split(" ") //Splits the numbers with spaces between into individual lines
            .map(|component| component.parse::<u64>().unwrap()) //Converts all the numbers into integers
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