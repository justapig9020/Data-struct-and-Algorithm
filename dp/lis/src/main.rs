fn main() {
    let test = [3, 1, 4, 1, 5, 9, 10, 2, 6, 5, 3];
    let lis = lis(&test);
    println!("{}", lis);
}

use std::cmp;
fn lis(s: &[i32]) -> usize {
    let mut dp = vec![1 as usize; s.len()];
    let mut max = 0;
    for (i, n) in s.iter().enumerate() {
        for j in 0..i {
            if *n > s[j] {
                dp[i] = cmp::max(dp[i], dp[j] + 1);
            }
        }
        max = cmp::max(max, dp[i]);
    }
    max
}
