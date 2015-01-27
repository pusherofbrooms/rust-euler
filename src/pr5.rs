/// project euler problem 5
///lowest number with factors from 1 to 20

pub fn run () -> i32 {
    // we need enough of the least common factors to account for 1-20.
    // 3*3 is 9, 2^4 is 16.
    let factors = [5,7,9,11,13,16,17,19];
    let y=factors.iter().fold(1, |product , x| product * *x);
    return y;
}
