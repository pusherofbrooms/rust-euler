/// problem 1
/// sum of all multiples of 3 and 5 below 1000

pub fn run () -> i32 {
    range(0,1000)
        .filter(|&x| (x % 3 == 0) || (x % 5 == 0))
        .fold(0,|sum, x| sum + x)
}
