// Description: https://leetcode.cn/problems/trapping-rain-water/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // The accumulate count of water.
        let mut ac: i32 = 0;
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        let mut left_h: i32 = 0;
        let mut right_h: i32 = 0;

        while left < right {
            if height[left] > left_h {
                left_h = height[left];
            }
            if height[right] > right_h {
                right_h = height[right];
            }
            if left_h < right_h {
                ac += left_h - height[left];
                left += 1;
            }  else {
                ac += right_h - height[right];
                right -= 1;
            }
        }
        return ac
    }
}

fn main() {
    let heights: Vec<i32> = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let value = Solution::trap(heights);
    assert_eq!(6, value, "value not right");
}