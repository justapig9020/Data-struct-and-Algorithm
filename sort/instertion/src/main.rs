fn main() {
    let mut arr = [5, 3, 4, 2, 1];
    sort(&mut arr);
    println!("{:?}", arr);
}

fn sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j-1);
            j -= 1;
        }
    }
}
