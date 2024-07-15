// Description: https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len () {
            return vec![]
        }

        let mut res: Vec<i32> = vec![];

        let mut array_p: Vec<usize> = vec![0;26];
        let mut array_s: Vec<usize> = vec![0;26];

        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();

        for i in 0..p.len() {
            array_p[p_chars[i] as usize - 'a' as usize] += 1;
            array_s[s_chars[i] as usize - 'a' as usize] += 1;
        }

        if array_p == array_s {
            res.push(0);
        }

        for i in 0..s.len() - p.len() {
            array_s[s_chars[i] as usize - 'a' as usize] -= 1;
            array_s[s_chars[i+p.len()] as usize - 'a' as usize] += 1;
            if array_p == array_s {
                res.push(i as i32 +1);
            }
        }

        return res
    }
}

fn main() {
    let s: String = String::from("cbaebabacd");
    let p: String = String::from("abc");
    assert_eq!(vec![0,6], Solution::find_anagrams(s, p), "value not right");
}
