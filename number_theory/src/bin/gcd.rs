use std::cmp::{max, min};

fn main() {
    let n = 1029;
    let m = 1071;
    let gcd = euclidean_algorithm(n, m);
    println!("The gcd of {} and {} is {}", n, m, gcd);
}

/// Time complexity: O(log(b)) (b is smaller number)
/// Space complexity: O(1)
fn euclidean_algorithm(n: u64, m: u64) -> u64 {
    let mut a = max(n, m);
    let mut b = min(n, m);

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}