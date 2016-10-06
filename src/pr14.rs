/// Project Euler problem 14
/// find the number under 1 million with the longest Collatz sequence
/// n = n / 2 where n is even
/// n = (n * 3) + 1 where n is odd.
/// stop when you get to one.
///
/// I could use a hash map to store previous collatz sequences
/// and then compare num against the map as we go through the
/// sequence, but the naive way is quite fast in rust.

pub fn run () -> i32 {
    let mut highest_terms = 1;
    let mut highest_num = 1;
    for x in 2 .. 1000001 {
        let terms = collatz_terms(x as i64);
        if terms > highest_terms {
            highest_terms = terms;
            highest_num = x
        }
    }
    highest_num
}

fn collatz_terms (mut num:i64) -> i32 {
    let mut terms = 1;
    while num > 1 {
        if num % 2 == 0{
            num /= 2;
        } else {
            num = num*3 + 1;
        }
        terms += 1;
    }
    terms
}
