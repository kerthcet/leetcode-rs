// Description: https://leetcode.cn/problems/group-anagrams/?envType=study-plan-v2&envId=top-100-liked
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        strs.iter().for_each(|str| {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            map.entry(chars).or_insert(vec![]).push(str.clone());
        });
        map.into_iter().map(|(_, v)| v).collect()
    }
}

fn main() {
    let strs: Vec<String> = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
        ];
    let nums: Vec<Vec<String>> = Solution::group_anagrams(strs);
    // Fetching values from hashmap is orderless.
    assert_eq!(3, nums.len(), "nums is not right");
}