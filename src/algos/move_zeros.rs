// Description: https://leetcode.cn/problems/move-zeroes/description/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut m: usize = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                (nums[i], nums[m]) = (nums[m], nums[i]);
                m += 1;
            }
        }
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![4,2,4,0,0,3,0,5,1,0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(vec![4,2,4,3,5,1,0,0,0,0], nums, "nums not right");
}