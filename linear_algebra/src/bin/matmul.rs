use std::vec;

fn main() {
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0], 
    ];

    let b = vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0], 
    ];

    let result = matmul(&a, &b);
    println!("Result of matrix multiplication:");
    for row in result {
        for value in row {
            print!("{:.1} ", value);
        }
        println!();
    }
}

/// Time complexity: O(nmp)
/// Space complexity: O(np)
fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let row_a = a.len();
    let col_a = a[0].len();
    let row_b = b.len();
    let col_b = b[0].len();
    assert_eq!(col_a, row_b, "Matrix dimensions do not match for multiplication");

    let mut result = vec![vec![0.0; col_b]; row_a];
    for i in 0..row_a {
        for j in 0..col_b {
            for k in 0..col_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}