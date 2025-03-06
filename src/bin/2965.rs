use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cnt = HashMap::new();
        for row in grid.iter() {
            for val in row.iter() {
                *cnt.entry(val).or_insert( 0) += 1;
            }
        }


        let r = grid.len() as i32;
        let n = r * r;
        let mut missing = 0;
        let mut repeated = 0;
        for i in 1..=n{
            if cnt.get(&i).is_none(){
                missing = i;
            } else if *cnt.get(&i).unwrap() == 2{
                repeated = i;
            }
        }
        vec![repeated, missing]
    }
}

fn main(){
    Solution::find_missing_and_repeated_values(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
}
