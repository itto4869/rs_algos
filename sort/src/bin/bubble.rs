fn main() {
    let mut arr = vec![4, 100, 2, 6, 44];

    println!("Unsorted: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted: {:?}", arr);
}



/// Time complexity: O(n^2)
/// Space complexity: O(1)
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}