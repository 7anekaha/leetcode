use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.into_iter().enumerate() {
            match seen.contains_key(&(target - n)) {
                true => {
                    return vec![i as i32, *seen.get(&(target - n)).unwrap() as i32];
                }
                false => {
                    seen.insert(n, i);
                }
            }
        }
        vec![]
    }
}
