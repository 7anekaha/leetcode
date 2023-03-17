use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut sorted_string: Vec<char> = s.chars().collect::<Vec<char>>();
            sorted_string.sort_by(|a, b| a.cmp(b));
            let sorted_string: String = sorted_string.iter().collect();

            map.entry(sorted_string).or_insert(vec![]).push(s);
        }

        let results: Vec<Vec<String>> = map.into_values().collect();
        results
    }
}
