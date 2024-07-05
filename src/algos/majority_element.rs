// Description: https://leetcode.cn/problems/majority-element/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

// Inspired by Boyer-Moore Voting Algorithm.
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut v = nums[0];
        let mut c = 1;

        for i in 1..nums.len() {
            if nums[i] == v {
                c += 1;
            }else {
                c -= 1;
                if c == 0 {
                    // i+1 couldn't be nums.len() here, so it's safe.
                    v = nums[i+1];
                    c = 0;
                }
            }
        }
        return v
    }
}

fn main() {
    let nums: Vec<i32> = vec![2,2,1,1,1,2,2];
    assert_eq!(2, Solution::majority_element(nums), "value not right");
}