fn main() {
    let a = [5.0, 12.0, 3.0, 7.0, 3.0, 1.0];
    let x = 3.2;
    let y = horner(&a, x);
    
    if let Some((first_coef, other_coefs)) = a.split_first() {
        for (i, coef) in other_coefs.iter().rev().enumerate() {
            print!("{}x^{} + ", coef, other_coefs.len() - i);
        }
        println!("{} = {}", first_coef, y);
    }
}

/// Time complexity: O(n) (n is degree of polynomial)
/// Space complexity: O(1)
fn horner(a: &[f64], x: f64) -> f64 {
    let mut y = 0.0;
    for coef in a.iter().rev() {
        y = y * x + coef;
    }
    y
}