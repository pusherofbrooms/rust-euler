/// project euler problem 7
/// the 10001th prome number

pub fn run () -> i32 {
    // prime the pump (puns haha)
    let mut primes = vec![2,3];
    let mut candidate = 4;
    let lastprime=10001;
    
    while primes.len() < lastprime {
        let mut primeflag=true;
        // we check each candidate only against previously found
        // prime numbers.
        for x in primes.iter() {
            if candidate % *x == 0 {
                primeflag=false;
                break;
            }
        }
        if primeflag {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes[primes.len()-1]
}
