fn partition(mut arr: Vec<i32>, p: usize, r: usize) -> (usize, Vec<i32>) {
    let x = arr[r - 1];
    let mut i = p - 1;
    for j in (p - 1)..(r - 1) {
        if arr[j] <= x {
            
            arr.swap(i, j);
            i = i + 1;
        }
    }
    arr.swap(i , r - 1);
    (i + 1, arr)
}

fn quick_sort(mut arr: Vec<i32>, p: usize, r: usize) -> Vec<i32> {
    if p < r {
        let q: usize;
        (q, arr) = partition(arr, p, r);
        arr = quick_sort(arr, p, q - 1);
        arr = quick_sort(arr, q + 1, r);
        return arr;
    } else {
        arr
    }
}

fn main() {
    let arr = vec![1, 20, 300, 4, 5, 60, 7, 8, 9];
    let arr = quick_sort(arr, 1, 9);
    println!("{:?}", arr)
}
