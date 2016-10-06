/// project euler problem 9
/// where a<b<c find the pythagorean triplet where a+b+c=1000 and produce the product abc

pub fn run () -> i32 {
    // if a < b < c, 332 is the highest a can go.
    for a in 1..333 {
        // this loop needs adjustment, but it works for this case.
        // where a==1 and b<c, b can't be higher than 499
        for b in a .. 499 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    return 0;
}
