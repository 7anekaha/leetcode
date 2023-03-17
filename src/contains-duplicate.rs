use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // let hash_set: HashSet<i32>  = nums.iter().cloned().collect();
        // nums.len() == hash_set.len()

        let mut hash_set: HashSet<i32> = HashSet::new();
        for n in nums.into_iter() {
            if hash_set.contains(&n) {
                return true;
            }
            hash_set.insert(n);
        }
        false
    }
}