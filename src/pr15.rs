/// project euler problem 15
/// from top left to bottom right, find all of the paths in a 20x20 grid
/// that only go down and right.
/// for this grid, it would be:
///    40!
///  -------
///  20! * 20!
/// wow, I think we'll need to use bignums.

extern crate num;
use num::{BigInt, One};
use num::bigint::ToBigInt;

pub fn run () -> BigInt {
    // we'll generalize to rectangular instead of just a square in case we need to use this later.
    let sizex = 20;
    let sizey = 20;
    let steps = sizex+sizey;
    let mut bigfact:BigInt = One::one();
    let mut smallfact:BigInt = One::one();
    let mut low;
    let mut high;
    if sizex > sizey {
        high=sizex;
        low=sizey;
    } else {
        high=sizey;
        low=sizex;
    }
    //calculate smaller factorial
    for x in (2..low+1) {
        smallfact = smallfact * x.to_bigint().unwrap();
    }
    //calculate the big factorial less the larger of the two sizes
    for x in (steps-high+1..steps+1){
        bigfact = bigfact * x.to_bigint().unwrap();
    }
    bigfact / smallfact
}
