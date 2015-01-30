/// project euler problem 6
///
/// Find the difference between the sum of the squares of the
/// first one hundred natural numbers and the square of the sum.

pub fn run() -> i32 {
    let num=100;
    //casting as float as a casting exercise.
    sq_of_sum(num as f32) - sum_of_sq(num)
}

fn sq_of_sum (n:f32) -> i32 {
    // sum of numbers 1 to n is (n+1)/2*n
    let sum=(n+1.0)/2.0*n;
    //casting as a casting exercise
    (sum*sum) as i32
}

fn sum_of_sq (n:i32) -> i32 {
    // I only know the naive way to do this
    let mut sum=0;
    for i in (1..n+1) {
        sum += i*i;
    }
    sum
}
