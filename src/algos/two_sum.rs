// Description: https://leetcode.cn/problems/two-sum/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    // brute-force enumeration
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        return vec![0, 0]
    }

    // Leverage hash table for O(1) search.
    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            let k = target - nums[i];
            match map.get(&k) {
                Some(v) => {
                    return vec![i as i32, *v as i32]
                }
                None => {
                    map.insert(nums[i], i);
                }
            }
        }
        return vec![0,0]
    }
}

fn main() {
    let nums = vec![2,7,11,15];
    let value = Solution::two_sum_2(nums, 9);
    assert_eq!(vec![1, 0], value, "value not right");
}