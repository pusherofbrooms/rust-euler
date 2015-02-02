/// project euler problem 12
/// find the first triangular with over 500 divisors
/// a triangular number is, for example 1+2+3+4+5+6=21
///
/// We can assume that 1 and the number itself are divisors,
/// so we are really lookig for over 498 divisors.
///
/// We don't need to search further than sqrt of the number.

use std::num;
use std::iter;

pub fn run () -> i32 {
    let maxdiv = 500;
    let mut tri = 3;
    let mut sum = 6;
    // we will return from in the loop, so let's make it Apple's address.
    loop {
        // get our first sum
        tri += 1;
        sum += tri;
        let mut factors = 0;
        let mut tmp = sum;
        let root = num::Float::floor(num::Float::sqrt(sum as f64)) as i32;
        
        for x in (2..root+1) {
            if sum % x == 0 {
                factors +=2
            }
        }
        if factors+2 > maxdiv {return sum;}
    }
}


    
