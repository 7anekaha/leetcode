use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map: HashMap<char, i32> = HashMap::new();

        for (s_char, t_char) in s.chars().zip(t.chars()){
            *map.entry(s_char).or_insert(0) += 1;
            *map.entry(t_char).or_insert(0) -= 1;
        }

        !map.iter().any(|(_, &v)| v != 0)
    }
}
