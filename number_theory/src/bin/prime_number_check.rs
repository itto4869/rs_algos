fn main() {
    let number = 13023;
    if prime_number_check(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}

/// Time complexity: O(n)
/// Space complexity: O(1)
fn prime_number_check(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}