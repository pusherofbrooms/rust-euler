/// project euler problem 3
/// largest prime factor of 600851475143

pub fn run() -> u64 {
    let mut target: u64 = 600851475143;
    let mut largest_factor: u64 = 1;
    let mut factor_candidate:u64 = 2;
    
    while target != 1 {
        while target % factor_candidate == 0 {
            largest_factor = factor_candidate;
            target = target / factor_candidate;
        }
        factor_candidate+=1;
    }
    return largest_factor;
}
