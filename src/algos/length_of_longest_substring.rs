// Description: https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&envId=top-100-liked
use std::collections::HashMap;

struct Solution;



impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char,usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut len: usize = 0;
        let mut start: usize = 0;

        for i in 0..chars.len() {
            if let Some(v) = map.get(&chars[i]) {
                start = start.max(v+1);
            }
            map.insert(chars[i], i);
            len = len.max(i - start + 1);
        }
        return len as i32
    }
}

fn main() {
    let s: String = String::from("abba");
    assert_eq!(Solution::length_of_longest_substring(s), 2);
}