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
        .parse::<u32>().unwrap();                               // assuming convertability, convert to unsigned integer

    let numbers_sequence = lines
        .next().unwrap() //Gets the second line of input with the numbers
        .split(" "); //Splits the numbers with spaces between into individual lines
    /*for number-string in numbers_sequence {
          println!("{}", number-string);
          Konvertera string till int
          Sortera storleksordning
    } */
      

    eprintln!("Kattis skips this comment!");
    println!("Print to standard output.");
}
