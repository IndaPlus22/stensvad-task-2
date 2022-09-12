/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
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
 
    let r = 30; 
    /*let r = lines 
        .next().unwrap()               // get first line of input
        .parse::<u32>().unwrap();      // assuming convertability, convert to unsigned integer */
    
    let rhalf;
    if r%2 == 0{
        rhalf = r/2;
    }else {
        rhalf = (r + 1)/2;
    }

    println!("{}", rhalf);
    
    let k = 30;
    /*let k = lines 
        .next().unwrap()               // get second line of input
        .parse::<u32>().unwrap();      // assuming convertability, convert to unsigned integer */

    let khalf;
    if k%2 == 0{
        khalf = k/2;
    }else {
        khalf = (k + 1)/2;
    }

    println!("{}", khalf);
 
    for x in 1..(rhalf+1){
        let mut message = String::from("");
        for y in 1..(k+1){
            if k%2 == 0 { //Om det är ett jämnt antal kolumner
                if x <= y {
                    if y > khalf{
                        message.push_str(&message.chars().rev().collect::<String>());
                        break;
                    } else {
                        if x > 9 {
                            message.push_str(".");
                        } else {
                            message.push_str(&(x).to_string());
                        }
                    }
                } else {
                    if y > 9 {
                        message.push_str(".");
                    } else {
                        message.push_str(&(y).to_string());
                    }
                }
            } else { //Om det är ett ojämnt antal kolumner
                if x <= y {
                    if y > khalf {
                        message.push_str(&message.chars().rev().collect::<String>());
                        message.remove(khalf - 1);
                        break;
                    } else {
                        if x > 9 {
                            message.push_str(".");
                        } else {
                            message.push_str(&(x).to_string());
                        }
                    } 
                } else {
                    if y > 9 {
                        message.push_str(".");
                    } else {
                        message.push_str(&(y).to_string());
                    }
                }
            }
        }
        println!("{}", message)
    }
    for x in (rhalf+1)..(r+1){
        let mut message = String::from("");
        for y in 1..(k+1){
            if k%2 == 0 {
                if r + 1 -x <= y {
                    if y > khalf{
                        message.push_str(&message.chars().rev().collect::<String>());
                        break;
                    } else {
                        if r + 1 - x > 9 {
                            message.push_str(".");
                        } else {
                            message.push_str(&(r + 1 - x).to_string());
                        }
                    }
                } else {
                    if y > 9 {
                        message.push_str(".");
                    } else {
                        message.push_str(&(y).to_string());
                    }
                }
            } else {
                if r + 1 -x <= y {
                    if y > khalf {
                        message.push_str(&message.chars().rev().collect::<String>());
                        message.remove(khalf - 1);
                        break;
                    } else {
                        if r + 1 - x > 9 {
                            message.push_str(".");
                        } else {
                            message.push_str(&(r + 1 - x).to_string());
                        }
                    }
                } else {
                    if y > 9 {
                        message.push_str(".");
                    } else {
                        message.push_str(&(y).to_string());
                    }
                }
            }
            
        }
        println!("{}", message)
        
    }
    
 }
