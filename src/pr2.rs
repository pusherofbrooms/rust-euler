/// Project Euler problem 2
/// Sum of even fibonacci numbers less than 4,000,001

pub fn run() -> i32 {
    let mut old_sum = 1;
    let mut sum = 2;
    let mut tot = 2;
    while sum <= 4000000 {
        let x = sum;
        sum = sum + old_sum;
        old_sum = x;
        if sum % 2 == 0 {tot+=sum;}
    }
    tot
}
