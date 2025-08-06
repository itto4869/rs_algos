fn main() {
    let mut arr = vec![4, 100, 2, 6, 44];

    println!("Unsorted: {:?}", arr);
    selection_sort(&mut arr);
    println!("Sorted: {:?}", arr);
}

/// Time complexity: O(n^2)
/// Space complexity: O(1)
fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i..n {
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }
        arr.swap(min_idx, i);
    }
}