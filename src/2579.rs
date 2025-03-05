impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut ans:i64 = 1;
        for i in 0..n {
            ans = ans + (4*i) as i64;
        }
        ans
    }
}
