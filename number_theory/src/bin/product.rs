use std::cmp::{max, min};

fn main() {
    let x = 45;
    let y = 19;
    let prod = russian(x, y);
    println!("{} * {} = {}", x, y, prod);
}

/// Time complexity: O(log(n)) (n is smaller number)
/// Space complexity: O(1)
fn russian(x: u64, y: u64) -> u64 {
    let mut a = min(x, y);
    let mut b = max(x, y);
    let mut prod = 0;
    while a > 0 {
        if a % 2 == 1 {
            prod += b;
        }
        a /= 2;
        b *= 2;
    }
    prod
}