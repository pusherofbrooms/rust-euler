/// project euler problem 10
/// sum of all primes below 2,000,000
///
/// The approach from problem 7 takes several minutes.
/// We'll see if we can do better.

pub fn run () -> i64 {
    // prime the pump
    let mut candidate = 3;
    let mut sum:i64 = 2;
    let lastprime = 2000000;

    while candidate < lastprime {
        if isprime(candidate) {
            sum += candidate as i64;
            // println!("{} {}",candidate,sum);
        }
        //
        candidate += 2;
    }
    sum
}

// 1 is not prime
// even numbers > 2 not prime
// numbers > 3 divisible by 3 not prime
// if a number has no factors below sqrt(number) it is prime
// all primes greater than 3 can be represented by 6*k (+/-) 1
fn isprime (n:i32) -> bool {
    if n == 1 {return false;}
    else if n < 4 {return true;}
    else if n % 2 == 0 {return false;}
    else if n < 9 {return true;}
    else if n % 3 == 0 {return false;}
    else {
        let root = f64::floor(f64::sqrt(n as f64)) as i32;
        // 6*k (+/-) 1 and all factors less than sqrt(n)
        let mut k = 5;
        while k <= root {
            if n % k == 0 {return false;}
            if n % (k + 2) == 0 {return false;}
            k += 6;
        }
    }
    return true;
}

               
