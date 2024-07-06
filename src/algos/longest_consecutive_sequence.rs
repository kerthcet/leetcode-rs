// Description: https://leetcode.cn/problems/longest-consecutive-sequence/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }

        nums.sort();

        let mut max: i32 = 1;
        let mut count: i32 = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                continue
            } else if nums[i] == nums[i-1] + 1 {
                count += 1;
                if count > max {
                    max = count
                }
            } else{
                count = 1;
            }
        }
        return max
    }
}

fn main() {
    let nums: Vec<i32> = vec![0,3,7,2,5,8,4,6,0,1];
    let value: i32 = Solution::longest_consecutive(nums);
    assert_eq!(value, 9, "value is not right");
}