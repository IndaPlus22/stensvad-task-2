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
     // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
     let mut lines = input 
         .lock()
         .lines()
         .map(|_line| _line.ok().unwrap());
 
         //Viola Söderlund
     let lenght_of_sequence = lines 
         .next().unwrap()                                        // get first line of input
         .parse::<u64>().unwrap();                               // assuming convertability, convert to unsigned integer
 
     let mut amount_of_numbers_to_add;
     if lenght_of_sequence % 2 == 0 {
         amount_of_numbers_to_add = lenght_of_sequence / 2;
     }
     else {
         amount_of_numbers_to_add = (lenght_of_sequence + 1) / 2;
     }
 
     let mut numbers_sequence = lines
         .next().unwrap() //Gets the second line of input with the numbers
         .split(" ") //Splits the numbers with spaces between into individual lines
         .map(|component| component.parse::<u64>().unwrap()) //Converts all the numbers into integers
         .collect::<Vec<u64>>(); 
     numbers_sequence.sort(); //Sort from smalest to largest number
     numbers_sequence.reverse(); //Reverse the order
 
     let mut sum: u64 = 0;
     for x in 0..amount_of_numbers_to_add {
         sum += numbers_sequence[x as usize];
     } 
       
 
     eprintln!("Kattis skips this comment!");
     println!("{}", sum);
 }
