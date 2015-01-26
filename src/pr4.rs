/// project euler problem 4
/// find the largest palindrome number made from the product of 2 three digit numbers

pub fn run () -> i32 {
    let mut highest_palindrome:i32=1;
    // too lazy to count from 100 to 999 since 900 to 999 likely has some palindromes
    for x in (900..999){
        for y in (x..999){
            let product = x*y;
            if ispalindrome(product) && product > highest_palindrome {
                highest_palindrome = product;
            }
        }
    }
    return highest_palindrome;
}

// if I were ever to do something else with palindrome numbers, I'd peel this out into a library
// and maybe add better error checking.
fn ispalindrome (num: i32) -> bool {
    let mut n=num;
    let mut rev=0;
    while n > 0 {
        // if the last digit is 0, this is not a palindrome number
        if n % 10 == 0 && rev == 0 {
            return false;
        }
        // collect digits in rev in reverse order
        rev = rev * 10;
        rev += n % 10;
        n = n / 10;
    }
    if rev == num {
        return true;
    } else {
        return false;
    }        
}
