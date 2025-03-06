struct Solution;
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less = Vec::new();
        let mut greater = Vec::new();
        let mut same = Vec::new();
        for n in nums.into_iter(){
            if n < pivot {
                less.push(n);
            } else if n > pivot {
                greater.push(n);
            } else {
                same.push(n);
            }
        }
        less.append(&mut same);
        less.append(&mut greater);
        less
    }
}

fn main(){
    Solution::pivot_array(vec![9, 12, 3, 5, 14, 10, 10], 10);
}
